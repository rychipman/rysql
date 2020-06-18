use crate::{parser::cst, result::Result};
use lalrpop_util::{lexer::Token, lalrpop_mod};

lalrpop_mod!(rysql);

pub type ParseError<'t> = lalrpop_util::ParseError<usize, Token<'t>, &'static str>;

pub fn parse(input: &str) -> Result<cst::Expr> {
	let select = rysql::SelectParser::new().parse(input)?;
	let expr = cst::Expr::Select(select);
	Ok(expr)
}
