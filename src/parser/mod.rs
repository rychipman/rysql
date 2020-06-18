pub mod cst;
mod lalrpop;

#[cfg(test)]
mod test;

use crate::result::Result;

pub use lalrpop::ParseError;

pub fn parse(input: &str) -> Result<cst::Expr> {
	lalrpop::parse(input)
}
