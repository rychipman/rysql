use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/rysql.pest"]
struct SqlParser;
