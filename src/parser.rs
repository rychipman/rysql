use crate::result::Result;

#[derive(Debug)]
pub enum Expr {
	Select(Select),
}

impl Expr {
	pub fn parse(input: &str) -> Result<Self> {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct Select {

}
