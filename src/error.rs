use std::fmt;
use std::io;
use url::ParseError;

#[derive(Debug)]
pub enum CustomError {
    Io(io::Error),
    Parse(ParseError),
}

impl From<io::Error> for CustomError {
    fn from(error: io::Error) -> Self {
        CustomError::Io(error)
    }
}

impl From<ParseError> for CustomError {
    fn from(error: ParseError) -> Self {
        CustomError::Parse(error)
    }
}

impl From<()> for CustomError {
    fn from(_: ()) -> Self {
        CustomError::Io(io::Error::new(io::ErrorKind::Other, "An error occurred"))
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Io(err) => write!(f, "IO error: {}", err),
            CustomError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl std::error::Error for CustomError {}
