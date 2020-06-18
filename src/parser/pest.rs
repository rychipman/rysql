use pest_derive::Parser;
use pest::{iterators::Pair, Parser};
use crate::{parser::cst, result::Result};

#[derive(Parser)]
#[grammar = "parser/rysql.pest"]
struct SqlParser;

pub fn parse(input: &str) -> Result<cst::Expr> {
	let mut pairs = SqlParser::parse(Rule::select, input)?;
	let select = process_select(pairs.next().unwrap());
	let expr = cst::Expr::Select(select);
	Ok(expr)
}

fn process_select(pair: Pair<Rule>) -> cst::Select {
	assert_eq!(pair.as_rule(), Rule::select);
	let select_exprs = pair.into_inner().next().unwrap();
	let exprs = select_exprs.into_inner().map(process_select_expr).collect();
	let from = None;
	cst::Select{exprs, from}
}

fn process_select_expr(pair: Pair<Rule>) -> cst::SelectExpr {
	println!("{:?}", pair);
	assert_eq!(pair.as_rule(), Rule::select_expr);
	let col_name: cst::ColumnName = pair.as_str().into();
	let expr = cst::Expr::Column(col_name);
	let alias = None;
	cst::SelectExpr{expr, alias}
}
