extern crate time;
use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(time::ParseError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::Parse(ref e) => write!(f, "Parse error: {}", e),            
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Parse(ref err) => Some(err),
        }
    }

}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<time::ParseError> for Error {
    fn from(err: time::ParseError) -> Self {
        Error::Parse(err)
    }
}

