use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidConfig,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConfig => f.write_str("board configuration is invalid"),
        }
    }
}
