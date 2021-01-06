use std::fmt;

#[derive(Debug)]
pub enum Error {
    CellOutOfBound,
    OpeningFlaggedCell,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CellOutOfBound => f.write_str("coordinate is out of bound"),
            Error::OpeningFlaggedCell => f.write_str("can't open a flagged cell"),
        }
    }
}
