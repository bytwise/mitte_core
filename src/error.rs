use std::fmt;


#[derive(Debug)]
pub enum Error {
    OutOfBounds,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OutOfBounds => write!(fmt, "range out of bounds"),
        }
    }
}

impl std::error::Error for Error {}
