use crate::ast;
use bson::{Bson, doc};

pub struct Aggregate {
	pub collection: Option<CollectionName>,
	pub pipeline: bson::Array,
}

impl Aggregate {
	fn one() -> Self {
		Aggregate {
			collection: None,
			pipeline: Vec::new(),
		}
	}

	fn collection(name: CollectionName) -> Self {
		Aggregate {
			collection: Some(name),
			pipeline: Vec::new(),
		}
	}

	pub fn print(&self) {
		println!("aggregate: {:?}", self.collection);
		let val: serde_json::Value = Bson::Array(self.pipeline.clone()).into();
		println!("{}", serde_json::to_string_pretty(&val).unwrap());
	}

	fn append(&mut self, doc: bson::Document) {
		self.pipeline.push(Bson::Document(doc));
	}
}

#[derive(Debug)]
pub struct CollectionName {
	name: String,
}

#[derive(Debug)]
pub struct FieldName {
	name: String,
}

pub fn to_mql(stage: ast::Stage) -> Aggregate {
	match stage {
		ast::Stage::Dual => dual_to_mql(),
		ast::Stage::Collection(cs) => collection_to_mql(cs),
		ast::Stage::Filter(fs) => filter_to_mql(fs),
		ast::Stage::Project(ps) => project_to_mql(ps),
	}
}

fn dual_to_mql() -> Aggregate {
	let mut agg = Aggregate::one();
	agg.append(doc!{"$dual": 1});
	agg
}

fn collection_to_mql(stage: ast::CollectionStage) -> Aggregate {
	Aggregate::collection(stage.name)
}

fn filter_to_mql(stage: ast::FilterStage) -> Aggregate {
	let mut agg = to_mql(*stage.source);
	let filter = expr_to_mql(stage.expr);
	agg.append(doc!{"$match": {"$expr": filter}});
	agg
}

fn project_to_mql(stage: ast::ProjectStage) -> Aggregate {
	unimplemented!()
}

fn expr_to_mql(expr: ast::Expr) -> Bson {
	match expr {
		ast::Expr::Null => Bson::Null,
		ast::Expr::Bool(b) => Bson::Boolean(b),
		ast::Expr::Int(i) => Bson::Int32(i),
		ast::Expr::Long(l) => Bson::Int64(l),
		ast::Expr::String(s) => Bson::String(s),
		_ => unimplemented!(),
	}
}
