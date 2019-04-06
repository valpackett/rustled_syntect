#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;
extern crate syntect;

use std::cell::RefCell;
use rustler::{Env, Term, NifResult, Encoder, ResourceArc};
use syntect::{
    parsing::{SyntaxSet, SyntaxReference, ParseState},
    html::{tokens_to_classed_spans, ClassStyle}
};

lazy_static! {
    static ref SYNTAX_SET: SyntaxSet =
        SyntaxSet::load_defaults_newlines();
}

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

rustler_export_nifs! {
    "Elixir.RustledSyntect.Nif",
    [
        ("new_highlighter", 1, new_highlighter),
        ("highlight_line", 2, highlight_line),
        ("finalize", 1, finalize),
    ],
    Some(on_load)
}

fn on_load(env: Env, _info: Term) -> bool {
    resource_struct_init!(ClassedStreamHlWrap, env);
    true
}

fn new_highlighter<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let lang: &str = args[0].decode()?;
    let syntax = &*SYNTAX_SET.find_syntax_by_name(lang).ok_or(rustler::Error::Atom("unknown_lang"))?;
    Ok(ResourceArc::new(ClassedStreamHlWrap(RefCell::new(ClassedStreamHl::new(syntax)))).encode(env))
}

fn highlight_line<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let hlw: ResourceArc<ClassedStreamHlWrap> = args[0].decode()?;
    let mut hl = hlw.0.borrow_mut();
    let line: &str = args[1].decode()?;
    Ok(hl.parse_html_for_line(line).encode(env))
}

fn finalize<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let hlw: ResourceArc<ClassedStreamHlWrap> = args[0].decode()?;
    let hl = hlw.0.borrow();
    Ok(hl.finalize().encode(env))
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
