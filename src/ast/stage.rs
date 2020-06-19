use crate::{mql::CollectionName, ast::{Expr,Tuple, Binding}, cst};

#[derive(Debug)]
pub enum Stage {
	Dual,
	Collection(CollectionStage),
	Filter(FilterStage),
	Project(ProjectStage),
}

impl From<cst::Statement> for Stage {
	fn from(stmt: cst::Statement) -> Self {
		let filter = Stage::Filter(FilterStage{
			expr: Expr::Null,
			source: Box::new(Stage::Dual),
		});
		Stage::Project(ProjectStage{
			tuple: Tuple{
				bindings: vec![
					Binding::new("a", Expr::Null),
				],
			},
			source: Box::new(filter),
		})
	}
}

#[derive(Debug)]
pub struct CollectionStage {
	pub name: CollectionName,
}

#[derive(Debug)]
pub struct FilterStage {
	pub expr: Expr,
	pub source: Box<Stage>,
}

#[derive(Debug)]
pub struct ProjectStage {
	pub tuple: Tuple,
	pub source: Box<Stage>,
}
