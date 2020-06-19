pub mod parser;
pub mod result;
pub mod mql;
pub mod ast;

use std::{process, io::{self, Write}};
use crate::{parser::cst, result::{Result, Error}};

fn main() {
	match repl() {
		Ok(()) => {},
		Err(e) => {
			println!("{}", e);
			process::exit(1);
		}
	}
}

fn repl() -> Result<()> {
	loop {
		print_prompt()?;
		let stmt = match read_stmt() {
			Ok(stmt) => stmt,
			Err(Error::UnknownCommand(e)) => {
				println!("{}", Error::UnknownCommand(e));
				continue
			},
			Err(Error::UnknownMetaCommand(e)) => {
				println!("{}", Error::UnknownMetaCommand(e));
				continue
			},
			Err(Error::Unimplemented(e)) => {
				println!("{}", Error::Unimplemented(e));
				continue
			},
			Err(Error::Parse(msg)) => {
				println!("{}", msg);
				continue
			}
			Err(e) => return Err(e),
		};
		exec_stmt(stmt)?;
	}
}

fn exec_stmt(stmt: Statement) -> Result<()> {
	match stmt {
		Statement::Meta(MetaCommand::Exit) => process::exit(0),
		Statement::NoOp => {},
		Statement::Parsed(stmt) => {
			let ast = ast::Stage::from(stmt);
			let agg = mql::to_mql(ast);
			agg.print();
		},
	};
	Ok(())
}

fn read_stmt() -> Result<Statement> {
	Statement::parse(read_input()?)
}

fn read_input() -> Result<String> {
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;
	Ok(input.trim().to_string())
}

fn print_prompt() -> Result<()> {
	print!("> ");
	io::stdout().flush()?;
	Ok(())
}


enum Statement {
	Meta(MetaCommand),
	NoOp,
	Parsed(cst::Statement),
}

impl Statement {
	fn parse(input: String) -> Result<Self> {
		if input.len() == 0 {
			Ok(Statement::NoOp)
		} else if &input[..1] == "." {
			let meta = MetaCommand::parse(input[1..].to_string())?;
			Ok(Statement::Meta(meta))
		} else {
			let stmt = parser::parse(&input)?;
			Ok(Statement::Parsed(stmt))
		}
	}
}

enum MetaCommand {
	Exit,
}

impl MetaCommand {
	fn parse(cmd: String) -> Result<Self> {
		match cmd.as_str() {
			"exit" => Ok(MetaCommand::Exit),
			other => Err(Error::UnknownMetaCommand(other.to_string())),
		}
	}
}
