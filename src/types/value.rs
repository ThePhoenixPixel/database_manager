use crate::types::{ColumnType, DbError, DbResult};

// Wrapper-Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBSmallInt(pub i16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBInteger(pub i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBBigInt(pub i64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBFloat(pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBDouble(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBBoolean(pub bool);

#[derive(Debug, Clone, PartialEq)]
pub struct DBText(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DBDate(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DBDatetime(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DBTimestamp(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DBJson(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DBBlob(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    SmallInt(DBSmallInt),
    Integer(DBInteger),
    BigInt(DBBigInt),
    Float(DBFloat),
    Double(DBDouble),
    Text(DBText),
    VarChar(DBText),
    Boolean(DBBoolean),
    Date(DBDate),
    DateTime(DBDatetime),
    Timestamp(DBTimestamp),
    Json(DBJson),
    Blob(DBBlob),
}


/// From impl
impl From<i16> for DBSmallInt {
    fn from(v: i16) -> Self { DBSmallInt(v) }
}

impl From<i32> for DBInteger {
    fn from(v: i32) -> Self { DBInteger(v) }
}

impl From<i64> for DBBigInt {
    fn from(v: i64) -> Self { DBBigInt(v) }
}

impl From<f32> for DBFloat {
    fn from(v: f32) -> Self { DBFloat(v) }
}

impl From<f64> for DBDouble {
    fn from(v: f64) -> Self { DBDouble(v) }
}

impl From<bool> for DBBoolean {
    fn from(v: bool) -> Self { DBBoolean(v) }
}

impl From<&str> for DBText {
    fn from(s: &str) -> Self { DBText(s.to_string()) }
}

impl From<String> for DBText {
    fn from(s: String) -> Self { DBText(s) }
}

impl From<&str> for DBDate {
    fn from(s: &str) -> Self { DBDate(s.to_string()) }
}

impl From<String> for DBDate {
    fn from(s: String) -> Self { DBDate(s) }
}

impl From<&str> for DBDatetime {
    fn from(s: &str) -> Self { DBDatetime(s.to_string()) }
}

impl From<String> for DBDatetime {
    fn from(s: String) -> Self { DBDatetime(s) }
}

impl From<&str> for DBTimestamp {
    fn from(s: &str) -> Self { DBTimestamp(s.to_string()) }
}

impl From<String> for DBTimestamp {
    fn from(s: String) -> Self { DBTimestamp(s) }
}

impl From<&str> for DBJson {
    fn from(s: &str) -> Self { DBJson(s.to_string()) }
}

impl From<String> for DBJson {
    fn from(s: String) -> Self { DBJson(s) }
}


impl From<Vec<u8>> for DBBlob {
    fn from(v: Vec<u8>) -> Self { DBBlob(v) }
}


impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_smallint(&self) -> Option<i16> {
        match self {
            Value::SmallInt(i) => Some(i.0),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i32> {
        match self {
            Value::Integer(i) => Some(i.0),
            _ => None,
        }
    }

    pub fn as_bigint(&self) -> Option<i64> {
        match self {
            Value::BigInt(i) => Some(i.0),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            Value::Float(f) => Some(f.0),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match self {
            Value::Double(f) => Some(f.0),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(s) | Value::VarChar(s) => Some(&s.0),
            Value::Json(s) => Some(&s.0),
            Value::Date(s) => Some(&s.0),
            Value::DateTime(s) => Some(&s.0),
            Value::Timestamp(s) => Some(&s.0),
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

    pub fn as_timestamp(&self) -> Option<&str> {
        match self {
            Value::Timestamp(ts) => Some(&ts.0),
            _ => None,
        }
    }

    pub fn as_json(&self) -> Option<&str> {
        match self {
            Value::Json(j) => Some(&j.0),
            _ => None,
        }
    }

    pub fn as_blob(&self) -> Option<&[u8]> {
        match self {
            Value::Blob(b) => Some(&b.0),
            _ => None,
        }
    }

    // Convenience method to convert any number to i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::SmallInt(i) => Some(i.0 as i64),
            Value::Integer(i) => Some(i.0 as i64),
            Value::BigInt(i) => Some(i.0),
            _ => None,
        }
    }

    // Convenience method to convert any float to f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(f.0 as f64),
            Value::Double(f) => Some(f.0),
            _ => None,
        }
    }
}

// From implementations for common types
impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::SmallInt(DBSmallInt(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Integer(DBInteger(value))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::BigInt(DBBigInt(value))
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(DBFloat(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(DBDouble(value))
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
    fn from_value(value: &Value) -> DbResult<Self> where Self: Sized;
}

impl DbType for DBSmallInt {
    fn column_type() -> ColumnType { ColumnType::SmallInt }
    fn to_value(self) -> Value { Value::SmallInt(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::SmallInt(i) => Ok(*i),
            Value::Integer(i) => Ok(DBSmallInt(i.0 as i16)),
            Value::BigInt(i) => Ok(DBSmallInt(i.0 as i16)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBSmallInt", value))),
        }
    }
}

impl DbType for DBInteger {
    fn column_type() -> ColumnType { ColumnType::Integer }
    fn to_value(self) -> Value { Value::Integer(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Integer(i) => Ok(*i),
            Value::SmallInt(i) => Ok(DBInteger(i.0 as i32)),
            Value::BigInt(i) => Ok(DBInteger(i.0 as i32)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBInteger", value))),
        }
    }
}

impl DbType for DBBigInt {
    fn column_type() -> ColumnType { ColumnType::BigInt }
    fn to_value(self) -> Value { Value::BigInt(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::BigInt(i) => Ok(*i),
            Value::Integer(i) => Ok(DBBigInt(i.0 as i64)),
            Value::SmallInt(i) => Ok(DBBigInt(i.0 as i64)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBBigInt", value))),
        }
    }
}

impl DbType for DBFloat {
    fn column_type() -> ColumnType { ColumnType::Float }
    fn to_value(self) -> Value { Value::Float(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Float(f) => Ok(*f),
            Value::Double(f) => Ok(DBFloat(f.0 as f32)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBFloat", value))),
        }
    }
}

impl DbType for DBDouble {
    fn column_type() -> ColumnType { ColumnType::Double }
    fn to_value(self) -> Value { Value::Double(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Double(f) => Ok(*f),
            Value::Float(f) => Ok(DBDouble(f.0 as f64)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBDouble", value))),
        }
    }
}

impl DbType for DBText {
    fn column_type() -> ColumnType { ColumnType::Text }
    fn to_value(self) -> Value { Value::Text(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Text(s) | Value::VarChar(s) => Ok(s.clone()),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBText", value))),
        }
    }
}

impl DbType for DBBoolean {
    fn column_type() -> ColumnType { ColumnType::Boolean }
    fn to_value(self) -> Value { Value::Boolean(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Boolean(b) => Ok(*b),
            Value::Integer(i) => Ok(DBBoolean(i.0 != 0)),
            Value::SmallInt(i) => Ok(DBBoolean(i.0 != 0)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBBoolean", value))),
        }
    }
}

impl DbType for DBDate {
    fn column_type() -> ColumnType { ColumnType::Date }
    fn to_value(self) -> Value { Value::Date(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Date(s) => Ok(s.clone()),
            Value::Text(s) => Ok(DBDate(s.0.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBDate", value))),
        }
    }
}

impl DbType for DBDatetime {
    fn column_type() -> ColumnType { ColumnType::DateTime }
    fn to_value(self) -> Value { Value::DateTime(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::DateTime(s) => Ok(s.clone()),
            Value::Text(s) => Ok(DBDatetime(s.0.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBDatetime", value))),
        }
    }
}

impl DbType for DBTimestamp {
    fn column_type() -> ColumnType { ColumnType::Timestamp }
    fn to_value(self) -> Value { Value::Timestamp(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Timestamp(s) => Ok(s.clone()),
            Value::Text(s) => Ok(DBTimestamp(s.0.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBTimestamp", value))),
        }
    }
}

impl DbType for DBJson {
    fn column_type() -> ColumnType { ColumnType::Json }
    fn to_value(self) -> Value { Value::Json(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Json(s) => Ok(s.clone()),
            Value::Text(s) => Ok(DBJson(s.0.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBJson", value))),
        }
    }
}

impl DbType for DBBlob {
    fn column_type() -> ColumnType { ColumnType::Blob }
    fn to_value(self) -> Value { Value::Blob(self) }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Blob(b) => Ok(b.clone()),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DBBlob", value))),
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
