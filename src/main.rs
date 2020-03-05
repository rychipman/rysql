use std::{process, io::{self, Write}, fmt};

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
			Err(e) => return Err(e),
		};
		exec_stmt(stmt)?;
	}
}

fn exec_stmt(stmt: Statement) -> Result<()> {
	match stmt {
		Statement::Meta(MetaCommand::Exit) => process::exit(0),
		Statement::NoOp => {},
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

enum Error {
	Io(io::Error),
	UnknownCommand(String),
	UnknownMetaCommand(String),
	Other(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Error::Io(e) => format!("io error: {}", e),
			Error::UnknownCommand(c) => format!("unknown command '{}'", c),
			Error::UnknownMetaCommand(c) => format!("unknown meta command '{}'", c),
			Error::Other(s) => format!("unexpected error: {}", s),
		})
	}
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Error::Io(e)
	}
}

type Result<T> = std::result::Result<T, Error>;

enum Statement {
	Meta(MetaCommand),
	NoOp,
}

impl Statement {
	fn parse(input: String) -> Result<Self> {
		if input.len() == 0 {
			Ok(Statement::NoOp)
		} else if &input[..1] == "." {
			let meta = MetaCommand::parse(input[1..].to_string())?;
			Ok(Statement::Meta(meta))
		} else {
			Err(Error::UnknownCommand(input))
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
