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
	pub filter: Option<Expr>,
}

#[derive(Debug)]
pub struct AliasedSelectExpr {
	pub select_expr: SelectExpr,
	pub alias: Option<ColumnName>,
}

#[derive(Debug)]
pub enum SelectExpr {
	Star,
	Expr(Expr),
}

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

#[derive(Debug)]
pub enum TableExpr {
	Table(TableName),
	Join(Join),
	Subquery(Box<Select>),
}

#[derive(Debug)]
pub struct TableName {
	name: String,
}

impl<S: Into<String>> From<S> for TableName {
	fn from(name: S) -> Self {
		let name: String = name.into();
		let normalized_name = name.to_lowercase();
		TableName{name: normalized_name}
	}
}

#[derive(Debug)]
pub struct Join {
	pub left: Box<TableExpr>,
	pub right: TableName,
	pub filter: Option<Expr>,
}
