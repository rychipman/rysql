use std::{io, fmt};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
	Io(io::Error),
	UnknownCommand(String),
	UnknownMetaCommand(String),
	Pest(String),
	Other(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Error::Io(e) => format!("io error: {}", e),
			Error::UnknownCommand(c) => format!("unknown command '{}'", c),
			Error::UnknownMetaCommand(c) => format!("unknown meta command '{}'", c),
			Error::Pest(msg) => format!("parse error:\n{}", msg),
			Error::Other(s) => format!("unexpected error: {}", s),
		})
	}
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Error::Io(e)
	}
}

impl<R: pest::RuleType> From<pest::error::Error<R>> for Error {
	fn from(e: pest::error::Error<R>) -> Self {
		Error::Pest(format!("{}", e))
	}
}
