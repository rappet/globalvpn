//! Error which can be send to another node

use thiserror::Error;

pub type ProtocolResult<T> = Result<T, ProtocolError>;

#[derive(Error, Debug, Clone)]
pub enum ProtocolError {
    #[error("Unexpected EOF")]
    UnexpectedEof,
}
