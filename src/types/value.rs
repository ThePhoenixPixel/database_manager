use crate::types::{ColumnType, DbError, DbResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBInt(pub i64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBUInt(pub u64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBFloat(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBBoolean(pub bool);

#[derive(Debug, Clone, PartialEq)]
pub struct DBText(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DBVarChar {
    pub value: String,
    pub max_length: usize,
}

impl DBVarChar {
    pub fn new(value: String, max_length: usize) -> DbResult<Self> {
        if value.len() > max_length {
            Err(DbError::InvalidData(format!(
                "String length {} exceeds maximum {}",
                value.len(),
                max_length
            )))
        } else {
            Ok(DBVarChar { value, max_length })
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn max_length(&self) -> usize {
        self.max_length
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DBDate(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DBDatetime(pub String);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBTimestamp(pub i64);

#[derive(Debug, Clone, PartialEq)]
pub struct DBBlob(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Int(DBInt),
    UInt(DBUInt),
    Float(DBFloat),
    Text(DBText),
    VarChar(DBVarChar),
    Boolean(DBBoolean),
    Date(DBDate),
    DateTime(DBDatetime),
    Timestamp(DBTimestamp),
    Blob(DBBlob),
}

/// From impl
impl From<i16> for DBInt {
    fn from(v: i16) -> Self {
        DBInt(v as i64)
    }
}

impl From<i32> for DBInt {
    fn from(v: i32) -> Self {
        DBInt(v as i64)
    }
}

impl From<i64> for DBInt {
    fn from(v: i64) -> Self {
        DBInt(v)
    }
}

impl From<u16> for DBUInt {
    fn from(v: u16) -> Self {
        DBUInt(v as u64)
    }
}

impl From<u32> for DBUInt {
    fn from(v: u32) -> Self {
        DBUInt(v as u64)
    }
}

impl From<u64> for DBUInt {
    fn from(v: u64) -> Self {
        DBUInt(v)
    }
}

impl From<f32> for DBFloat {
    fn from(v: f32) -> Self {
        DBFloat(v as f64)
    }
}

impl From<f64> for DBFloat {
    fn from(v: f64) -> Self {
        DBFloat(v)
    }
}

impl From<bool> for DBBoolean {
    fn from(v: bool) -> Self {
        DBBoolean(v)
    }
}

impl From<&str> for DBText {
    fn from(s: &str) -> Self {
        DBText(s.to_string())
    }
}

impl From<String> for DBText {
    fn from(s: String) -> Self {
        DBText(s)
    }
}

impl From<&str> for DBDate {
    fn from(s: &str) -> Self {
        DBDate(s.to_string())
    }
}

impl From<String> for DBDate {
    fn from(s: String) -> Self {
        DBDate(s)
    }
}

impl From<&str> for DBDatetime {
    fn from(s: &str) -> Self {
        DBDatetime(s.to_string())
    }
}

impl From<String> for DBDatetime {
    fn from(s: String) -> Self {
        DBDatetime(s)
    }
}

impl From<i64> for DBTimestamp {
    fn from(v: i64) -> Self {
        DBTimestamp(v)
    }
}

impl From<Vec<u8>> for DBBlob {
    fn from(v: Vec<u8>) -> Self {
        DBBlob(v)
    }
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(i.0),
            _ => None,
        }
    }

    pub fn as_uint(&self) -> Option<u64> {
        match self {
            Value::UInt(i) => Some(i.0),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(f.0),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(&s.0),
            _ => None,
        }
    }

    pub fn as_varchar(&self) -> Option<&DBVarChar> {
        match self {
            Value::VarChar(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(b.0),
            _ => None,
        }
    }

    pub fn as_date(&self) -> Option<&str> {
        match self {
            Value::Date(d) => Some(&d.0),
            _ => None,
        }
    }

    pub fn as_datetime(&self) -> Option<&str> {
        match self {
            Value::DateTime(dt) => Some(&dt.0),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<i64> {
        match self {
            Value::Timestamp(ts) => Some(ts.0),
            _ => None,
        }
    }

    pub fn as_blob(&self) -> Option<&[u8]> {
        match self {
            Value::Blob(b) => Some(&b.0),
            _ => None,
        }
    }
}

// From implementations for common types
impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Int(DBInt(value as i64))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(DBInt(value as i64))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(DBInt(value))
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::UInt(DBUInt(value as u64))
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::UInt(DBUInt(value as u64))
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::UInt(DBUInt(value))
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(DBFloat(value as f64))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(DBFloat(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Text(DBText(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Text(DBText(value.to_string()))
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(DBBoolean(value))
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Blob(DBBlob(value))
    }
}

pub trait DbType {
    fn column_type() -> ColumnType;
    fn to_value(self) -> Value;
    fn from_value(value: &Value) -> DbResult<Self>
    where
        Self: Sized;
}

impl DbType for DBInt {
    fn column_type() -> ColumnType {
        ColumnType::Int
    }
    fn to_value(self) -> Value {
        Value::Int(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Int(i) => Ok(*i),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBInt",
                value
            ))),
        }
    }
}

impl DbType for DBUInt {
    fn column_type() -> ColumnType {
        ColumnType::UInt
    }
    fn to_value(self) -> Value {
        Value::UInt(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::UInt(i) => Ok(*i),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBUInt",
                value
            ))),
        }
    }
}

impl DbType for DBFloat {
    fn column_type() -> ColumnType {
        ColumnType::Float
    }
    fn to_value(self) -> Value {
        Value::Float(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Float(f) => Ok(*f),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBFloat",
                value
            ))),
        }
    }
}

impl DbType for DBText {
    fn column_type() -> ColumnType {
        ColumnType::Text
    }
    fn to_value(self) -> Value {
        Value::Text(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Text(s) => Ok(s.clone()),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBText",
                value
            ))),
        }
    }
}

impl DbType for DBVarChar {
    fn column_type() -> ColumnType {
        ColumnType::VarChar(255) // Default max length
    }
    fn to_value(self) -> Value {
        Value::VarChar(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::VarChar(v) => Ok(v.clone()),
            Value::Text(s) => DBVarChar::new(s.0.clone(), 255),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBVarChar",
                value
            ))),
        }
    }
}

impl DbType for DBBoolean {
    fn column_type() -> ColumnType {
        ColumnType::Boolean
    }
    fn to_value(self) -> Value {
        Value::Boolean(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Boolean(b) => Ok(*b),
            Value::Int(i) => Ok(DBBoolean(i.0 != 0)),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBBoolean",
                value
            ))),
        }
    }
}

impl DbType for DBDate {
    fn column_type() -> ColumnType {
        ColumnType::Date
    }
    fn to_value(self) -> Value {
        Value::Date(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Date(s) => Ok(s.clone()),
            Value::Text(s) => Ok(DBDate(s.0.clone())),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBDate",
                value
            ))),
        }
    }
}

impl DbType for DBDatetime {
    fn column_type() -> ColumnType {
        ColumnType::DateTime
    }
    fn to_value(self) -> Value {
        Value::DateTime(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::DateTime(s) => Ok(s.clone()),
            Value::Text(s) => Ok(DBDatetime(s.0.clone())),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBDatetime",
                value
            ))),
        }
    }
}

impl DbType for DBTimestamp {
    fn column_type() -> ColumnType {
        ColumnType::Timestamp
    }
    fn to_value(self) -> Value {
        Value::Timestamp(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Timestamp(s) => Ok(*s),
            Value::Int(s) => Ok(DBTimestamp(s.0)),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBTimestamp",
                value
            ))),
        }
    }
}

impl DbType for DBBlob {
    fn column_type() -> ColumnType {
        ColumnType::Blob
    }
    fn to_value(self) -> Value {
        Value::Blob(self)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Blob(b) => Ok(b.clone()),
            _ => Err(DbError::InvalidData(format!(
                "Cannot convert {:?} to DBBlob",
                value
            ))),
        }
    }
}

// Option<T> support
impl<T: DbType> DbType for Option<T> {
    fn column_type() -> ColumnType {
        T::column_type()
    }
    fn to_value(self) -> Value {
        match self {
            Some(v) => v.to_value(),
            None => Value::Null,
        }
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Null => Ok(None),
            _ => Ok(Some(T::from_value(value)?)),
        }
    }
}