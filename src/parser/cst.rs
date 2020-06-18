#[derive(Debug)]
pub enum Statement {
	Select(Select),
}

#[derive(Debug)]
pub enum Expr {
	Column(ColumnName),
}

#[derive(Debug)]
pub struct Select {
	pub exprs: Vec<SelectExpr>,
	pub from: Option<TableExpr>,
}

#[derive(Debug)]
pub struct SelectExpr {
	pub expr: Expr,
	pub alias: Option<ColumnName>,
}

#[derive(Debug)]
pub enum TableExpr {}

#[derive(Debug)]
pub struct ColumnName {
	name: String,
}

impl<S: Into<String>> From<S> for ColumnName {
	fn from(name: S) -> Self {
		let name: String = name.into();
		let normalized_name = name.to_lowercase();
		ColumnName{name: normalized_name}
	}
}
