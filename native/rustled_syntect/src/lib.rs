extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
extern crate syntect;

use std::cell::RefCell;
use rustler::{Env, Term, NifResult, ResourceArc};
use syntect::{
    parsing::{SyntaxSet, SyntaxReference, ParseState},
    html::{tokens_to_classed_spans, ClassStyle}
};

lazy_static! {
    static ref SYNTAX_SET: SyntaxSet =
        SyntaxSet::load_defaults_newlines();
}

mod atoms {
    rustler::atoms! {
        ok,
    }
}

rustler::init!(
    "Elixir.RustledSyntect.Nif",
    [
         new_highlighter,
         highlight_line,
         finalize,

        langs
    ],
    load = on_load);

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(ClassedStreamHlWrap, env);
    true
}

#[rustler::nif]
fn new_highlighter(lang: &str) -> NifResult<ResourceArc<ClassedStreamHlWrap>> {
    let syntax = &*SYNTAX_SET.find_syntax_by_name(lang).ok_or(rustler::Error::Atom("unknown_lang"))?;
    Ok(ResourceArc::new(ClassedStreamHlWrap(RefCell::new(ClassedStreamHl::new(syntax)))))
}

#[rustler::nif]
fn highlight_line(hlw: ResourceArc<ClassedStreamHlWrap>, line: &str) -> NifResult<String> {
    let mut hl = hlw.0.borrow_mut();
    Ok(hl.parse_html_for_line(line))
}

#[rustler::nif]
fn finalize(hlw: ResourceArc<ClassedStreamHlWrap>) -> NifResult<Vec<&'static str>> {
    let hl = hlw.0.borrow();
    Ok(hl.finalize())
}

#[rustler::nif]
fn langs() -> NifResult<Vec<SyntaxData>> {
    Ok((*SYNTAX_SET).syntaxes().iter().map(|s| {
        SyntaxData {
            name: s.name.clone(),
            file_extensions: s.file_extensions.clone(),
            first_line_match: s.first_line_match.clone(),
        }
    }).collect::<Vec<_>>())
}

#[derive(NifStruct)]
#[module = "RustledSyntect.Syntax"]
struct SyntaxData {
    name: String,
    file_extensions: Vec<String>,
    first_line_match: Option<String>,
}

struct ClassedStreamHl {
    open_spans: isize,
    parse_state: ParseState,
}

struct ClassedStreamHlWrap(RefCell<ClassedStreamHl>);

unsafe impl Send for ClassedStreamHlWrap {}
unsafe impl Sync for ClassedStreamHlWrap {}

impl ClassedStreamHl {
    pub fn new(syntax_reference: &SyntaxReference) -> ClassedStreamHl {
        ClassedStreamHl {
            open_spans: 0,
            parse_state: ParseState::new(syntax_reference),
        }
    }

    pub fn parse_html_for_line(&mut self, line: &str) -> String {
        let parsed_line = self.parse_state.parse_line(line, &*SYNTAX_SET);
        let (formatted_line, delta) = tokens_to_classed_spans(
            line,
            parsed_line.as_slice(),
            ClassStyle::Spaced);
        self.open_spans += delta;
        formatted_line
    }

    pub fn finalize(&self) -> Vec<&'static str> {
        let mut iolist = Vec::with_capacity(self.open_spans as usize);
        for _ in 0..self.open_spans {
            iolist.push("</span>");
        }
        iolist
    }
}
