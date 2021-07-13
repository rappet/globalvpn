//! Error which can be send to another node

use thiserror::Error;
use std::error::Error;
use std::fmt::Display;

pub type ProtocolResult<T> = Result<T, ProtocolError>;

#[derive(Error, Debug, Clone)]
pub enum ProtocolError {
    #[error("Unexpected EOF")]
    UnexpectedEof,
}
