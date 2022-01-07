extern crate rustler;
#[macro_use] extern crate rustler_codegen;
extern crate lazy_static;
extern crate syntect;

use std::cell::RefCell;
use rustler::{Env, Term, NifResult, ResourceArc};
use syntect::{
    parsing::{SyntaxSet, SyntaxReference, ParseState},
    html::{tokens_to_classed_spans, ClassStyle}
};

mod atoms {
    rustler::atoms! {
        ok,
    }
}

rustler::init!(
    "Elixir.RustledSyntect.Nif",
    [
        new_syntax_set,
        new_highlighter,
        highlight_line,
        finalize,

        langs
    ],
    load = on_load);

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(ClassedStreamHlWrap, env);
    rustler::resource!(SyntaxSetWrap, env);
    true
}

#[rustler::nif]
fn new_syntax_set(folder: Option<&str>) -> NifResult<ResourceArc<SyntaxSetWrap>> {
    let ss = make_syntax_set(folder);
    Ok(ResourceArc::new(SyntaxSetWrap(RefCell::new(SyntaxSetE::new(&ss)))))
}


#[rustler::nif]
fn new_highlighter(ss: ResourceArc<SyntaxSetWrap>, lang: &str) -> NifResult<ResourceArc<ClassedStreamHlWrap>> {
    let s = ss.0.borrow();
    let syntax = s.syntax_set.find_syntax_by_name(lang).ok_or(rustler::Error::Atom("unknown_lang"))?;
    Ok(ResourceArc::new(ClassedStreamHlWrap(RefCell::new(ClassedStreamHl::new(syntax, &s.syntax_set)))))
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
fn langs(folder: Option<&str>) -> NifResult<Vec<SyntaxData>> {
    let ss = make_syntax_set(folder);
    Ok(ss.syntaxes().iter().map(|s| {
        SyntaxData {
            name: s.name.clone(),
            file_extensions: s.file_extensions.clone(),
            first_line_match: s.first_line_match.clone(),
        }
    }).collect::<Vec<_>>())
}

fn make_syntax_set(folder: Option<&str>) -> SyntaxSet {
    let mut ss = SyntaxSet::load_defaults_newlines();
    if let Some(f) = folder {
        let mut builder = ss.into_builder();
        builder.add_from_folder(f, false).unwrap();
        ss = builder.build()
    };
    ss
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
    syntax_set: SyntaxSet,
}

struct ClassedStreamHlWrap(RefCell<ClassedStreamHl>);

unsafe impl Send for ClassedStreamHlWrap {}
unsafe impl Sync for ClassedStreamHlWrap {}

impl ClassedStreamHl {
    pub fn new(syntax_reference: &SyntaxReference, syntax_set: &SyntaxSet) -> ClassedStreamHl {
        ClassedStreamHl {
            open_spans: 0,
            parse_state: ParseState::new(syntax_reference),
            syntax_set: syntax_set.clone(),
        }
    }

    pub fn parse_html_for_line(&mut self, line: &str) -> String {
        let parsed_line = self.parse_state.parse_line(line, &self.syntax_set);
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

struct SyntaxSetE {
    syntax_set: SyntaxSet,
}

struct SyntaxSetWrap(RefCell<SyntaxSetE>);

unsafe impl Send for SyntaxSetWrap {}
unsafe impl Sync for SyntaxSetWrap {}

impl SyntaxSetE {
    pub fn new(syntax_set: &SyntaxSet) -> SyntaxSetE {
        SyntaxSetE {
            syntax_set: syntax_set.clone(),
        }
    }
}
