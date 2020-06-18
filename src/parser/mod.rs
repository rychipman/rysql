pub mod cst;
mod pest;

use crate::result::Result;

pub fn parse(input: &str) -> Result<cst::Expr> {
	pest::parse(input)
}
