use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum DbError {
    ConnectionError(String),
    QueryError { table: Option<String>, message: String },
    SchemaError { table: Option<String>, message: String },
    InsertError { table: String, message: String },
    UpdateError { table: String, message: String },
    DeleteError { table: String, message: String },
    CreateTableError { table: String, message: String },
    DropTableError { table: String, message: String },
    NotFound(String),
    InvalidData(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            DbError::QueryError { table, message } => {
                if let Some(t) = table {
                    write!(f, "Query error on table '{}': {}", t, message)
                } else {
                    write!(f, "Query error: {}", message)
                }
            }
            DbError::SchemaError { table, message } => {
                if let Some(t) = table {
                    write!(f, "Schema error on table '{}': {}", t, message)
                } else {
                    write!(f, "Schema error: {}", message)
                }
            }
            DbError::InsertError { table, message } => {
                write!(f, "Insert error on table '{}': {}", table, message)
            }
            DbError::UpdateError { table, message } => {
                write!(f, "Update error on table '{}': {}", table, message)
            }
            DbError::DeleteError { table, message } => {
                write!(f, "Delete error on table '{}': {}", table, message)
            }
            DbError::CreateTableError { table, message } => {
                write!(f, "Create table error for '{}': {}", table, message)
            }
            DbError::DropTableError { table, message } => {
                write!(f, "Drop table error for '{}': {}", table, message)
            }
            DbError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DbError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl Error for DbError {}

pub type DbResult<T> = Result<T, DbError>;
