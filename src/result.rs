use std::{io, fmt};
use crate::parser;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	UnknownCommand(String),
	UnknownMetaCommand(String),
	Parse(String),
	Unimplemented(String),
	Other(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Error::Io(e) => format!("io error: {}", e),
			Error::UnknownCommand(c) => format!("unknown command '{}'", c),
			Error::UnknownMetaCommand(c) => format!("unknown meta command '{}'", c),
			Error::Parse(msg) => format!("parse error:\n{}", msg),
			Error::Unimplemented(msg) => format!("unimplemented: {}", msg),
			Error::Other(s) => format!("unexpected error: {}", s),
		})
	}
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Error::Io(e)
	}
}

impl From<parser::ParseError<'_>> for Error {
	fn from(e: parser::ParseError<'_>) -> Self {
		Error::Parse(format!("{}", e))
	}
}
