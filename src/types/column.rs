use crate::types::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    SmallInt,
    Integer,
    BigInt,
    Float,
    Double,
    Text,
    VarChar(usize),
    Boolean,
    Date,
    DateTime,
    Timestamp,
    Json,
    Blob,
}

impl ColumnType {
    pub fn type_name(&self) -> &'static str {
        match self {
            ColumnType::SmallInt => "SMALLINT",
            ColumnType::Integer => "INTEGER",
            ColumnType::BigInt => "BIGINT",
            ColumnType::Float => "FLOAT",
            ColumnType::Double => "DOUBLE",
            ColumnType::Text => "TEXT",
            ColumnType::VarChar(_) => "VARCHAR",
            ColumnType::Boolean => "BOOLEAN",
            ColumnType::Date => "DATE",
            ColumnType::DateTime => "DATETIME",
            ColumnType::Timestamp => "TIMESTAMP",
            ColumnType::Json => "JSON",
            ColumnType::Blob => "BLOB",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub column_type: ColumnType,
    pub nullable: bool,
    pub primary_key: bool,
    pub auto_increment: bool,
    pub unique: bool,
    pub default_value: Option<Value>,
}

impl Column {
    pub fn new(name: impl Into<String>, column_type: ColumnType) -> Self {
        Self {
            name: name.into(),
            column_type,
            nullable: false,
            primary_key: false,
            auto_increment: false,
            unique: false,
            default_value: None,
        }
    }

    pub fn nullable(mut self) -> Self {
        self.nullable = true;
        self
    }

    pub fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self
    }

    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    pub fn default(mut self, value: Value) -> Self {
        self.default_value = Some(value);
        self
    }
}
