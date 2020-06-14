#[derive(Debug)]
pub enum Expr {
	Select(Select),
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
pub struct ColumnName(String);

#[derive(Debug)]
pub enum TableExpr {}