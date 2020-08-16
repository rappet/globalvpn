use serde::export::Formatter;
use std::error::Error;
use std::fmt::Display;

pub type ProtocolResult<T> = Result<T, ProtocolError>;

#[derive(Debug, Clone)]
pub enum ProtocolError {
    UnexpectedEof,
}

impl ProtocolError {
    fn msg(&self) -> &'static str {
        match self {
            ProtocolError::UnexpectedEof => "unexpected eof",
        }
    }
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg())
    }
}

impl Error for ProtocolError {}
