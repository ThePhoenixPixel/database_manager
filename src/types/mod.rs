use std::collections::HashMap;

pub use crate::types::value::*;
pub use crate::types::column::*;
pub use crate::types::filter::*;
pub use crate::types::index::*;
pub use crate::types::table_schema::TableSchema;
pub use crate::types::alter_table::AlterTableOperation;
pub use crate::types::foreign_key::*;
pub use crate::types::error::*;

mod value;
mod column;
mod error;
mod table_schema;
mod filter;
mod index;
mod foreign_key;
mod alter_table;

pub type Row = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Like,
    In,
    Between,
    IsNull,
    IsNotNull,
}

#[derive(Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
}

impl JoinType {
    fn to_sql(&self) -> &'static str {
        match self {
            JoinType::Inner => "INNER",
            JoinType::Left => "LEFT",
            JoinType::Right => "RIGHT",
        }
    }
}

#[derive(Clone)]
pub struct JoinClause {
    pub table: String,
    pub left: String,
    pub right: String,
    pub join_type: JoinType,
}
