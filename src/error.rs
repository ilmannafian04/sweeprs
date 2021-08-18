use std::fmt;

/// General error type
#[derive(Debug)]
pub enum Error {
    InvalidConfigError,
    IndexOutOfBoundError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConfigError => f.write_str("board configuration is invalid"),
            Error::IndexOutOfBoundError => f.write_str("index is out of bound"),
        }
    }
}
