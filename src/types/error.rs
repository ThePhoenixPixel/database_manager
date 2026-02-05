use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DbError {
    ConnectionError(String),
    QueryError(String),
    SchemaError(String),
    NotFound(String),
    InvalidData(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            DbError::QueryError(msg) => write!(f, "Query error: {}", msg),
            DbError::SchemaError(msg) => write!(f, "Schema error: {}", msg),
            DbError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DbError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl Error for DbError {}

pub type DbResult<T> = Result<T, DbError>;
