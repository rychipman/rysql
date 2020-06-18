pub mod cst;
mod pest;
pub mod lalrpop;

use crate::result::Result;

pub fn parse(input: &str) -> Result<cst::Expr> {
	lalrpop::parse(input)
}
