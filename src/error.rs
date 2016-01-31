extern crate time;
use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub struct FormatError {
    pub format: String,
    pub position: usize,
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &FormatError{ref format, ref position} = self;
        write!(f, "Format Error in \"{}\" at {}", format, position)
    }
}

impl error::Error for FormatError {
    fn description(&self) -> &str {
        "format error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(time::ParseError),
    Format(FormatError),
    Config,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::Parse(ref e) => write!(f, "Parse error: {}", e),
            Error::Format(ref e) => write!(f, "Format error: {}", e),
            Error::Config => write!(f, "Config error"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Parse(ref e) => e.description(),
            Error::Format(ref e) => e.description(),
            Error::Config => "config error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Parse(ref e) => Some(e),
            Error::Format(ref e) => Some(e),
            Error::Config => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<time::ParseError> for Error {
    fn from(e: time::ParseError) -> Self {
        Error::Parse(e)
    }
}

impl From<FormatError> for Error {
    fn from(e: FormatError) -> Self {
        Error::Format(e)
    }
}
