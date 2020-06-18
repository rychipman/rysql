use crate::{ parser::cst, result::{Result, Error} };
use lalrpop_util::{lexer::Token, lalrpop_mod};

lalrpop_mod!(rysql);

pub type ParseError<'t> = lalrpop_util::ParseError<usize, Token<'t>, &'static str>;

pub fn parse(input: &str) -> Result<cst::Expr> {
	println!("{:?}", rysql::TermParser::new().parse(input));
	Err(Error::Unimplemented("lalrpop parsing".into()))
}
