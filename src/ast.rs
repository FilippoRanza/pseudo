use crate::pseudo_lang;
use lalrpop_util;

type ParseResult<'a> = Result<Code<'a>, lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'a>, &'static str>>;

fn parse<'a>(code: &'a str) -> ParseResult<'a> {
    let parser = pseudo_lang::CodeParser::new();
    parser.parse(code)
}

pub struct Code<'a> {
    pub caption: &'a str,
    pub code: Vec<Command<'a>>
}

pub enum Command<'a> {
    Declaration(DeclBlock<'a>),
    Assign(Assign<'a>),
    Condition(Condition<'a>),
    ForLoop(ForLoop<'a>),
    WhileLoop(ConditionPair<'a>),
    Return(&'a str)
}

pub type DeclBlock<'a> = Vec<Decl<'a>>;
pub type Decl<'a> = (&'a str, &'a str);

pub type Assign<'a> = (&'a str, &'a str);

pub struct Condition<'a> {
    pub if_block : ConditionPair<'a>,
    pub elif_blocks: Vec<ConditionPair<'a>>,
    pub else_block: Option<Vec<Command<'a>>>
}


pub struct ConditionPair<'a> {
    pub cond: &'a str, 
    pub body: Vec<Command<'a>>
}

pub struct ForLoop<'a> {
    pub kind: ForLoopKind<'a>,
    pub body: Vec<Command<'a>>
}

pub enum ForLoopKind<'a> {
    Count((&'a str, &'a str, &'a str)),
    Iter((&'a str, &'a str))
}


