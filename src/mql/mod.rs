use crate::ast;
use bson::doc;

pub struct Aggregate {
	pub collection: Option<CollectionName>,
	pub pipeline: Vec<bson::Document>,
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

	fn append(&mut self, doc: bson::Document) {
		self.pipeline.push(doc);
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
	unimplemented!()
}

fn project_to_mql(stage: ast::ProjectStage) -> Aggregate {
	unimplemented!()
}
