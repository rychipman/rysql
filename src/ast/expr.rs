use crate::mql::FieldName;

#[derive(Debug)]
pub enum Expr {
	Bool(bool),
	Int(i32),
	Long(i64),
	String(String),
	Array(Vec<Expr>),
	Tuple(Tuple),
	Null,
	Missing,
}

#[derive(Debug)]
pub struct Tuple {
	bindings: Vec<Binding>,
}

#[derive(Debug)]
pub struct Binding {
	name: FieldName,
	value: Box<Expr>,
}
