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
}

#[derive(Debug)]
pub struct Tuple {
	pub bindings: Vec<Binding>,
}

#[derive(Debug)]
pub struct Binding {
	pub name: FieldName,
	pub value: Box<Expr>,
}

impl Binding {
	pub fn new<S: Into<String>>(name: S, value: Expr) -> Self {
		Binding {
			name: FieldName::new(name),
			value: Box::new(value),
		}
	}
}
