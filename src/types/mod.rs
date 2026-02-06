use std::collections::HashMap;

pub use crate::types::value::*; // Export all DB types (DBInteger, DBText, etc.)
pub use crate::types::column::Column;
pub use crate::types::column::ColumnType;
pub use crate::types::filter::Filter;
pub use crate::types::filter::QueryFilters;
pub use crate::types::index::Index;
pub use crate::types::index::IndexType;
pub use crate::types::table_schema::TableSchema;
pub use crate::types::alter_table::AlterTableOperation;
pub use crate::types::foreign_key::ForeignKey;
pub use crate::types::foreign_key::ForeignKeyAction;
pub use crate::types::error::DbResult;
pub use crate::types::error::DbError;

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

