use crate::{mql::CollectionName, ast::{Expr,Tuple}};

#[derive(Debug)]
pub enum Stage {
	Dual,
	Collection(CollectionStage),
	Filter(FilterStage),
	Project(ProjectStage),
}

#[derive(Debug)]
pub struct CollectionStage {
	pub name: CollectionName,
}

#[derive(Debug)]
pub struct FilterStage {
	expr: Expr,
	source: Box<Stage>,
}

#[derive(Debug)]
pub struct ProjectStage {
	tuple: Tuple,
	source: Box<Stage>,
}
