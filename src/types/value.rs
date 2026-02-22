use chrono::Utc;
use crate::types::{ColumnType, DbError, DbResult};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DBInt(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DBUInt(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DBFloat(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DBBoolean(pub bool);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DBText(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DBVarChar {
    value: String,
    max_length: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DBDate(pub String);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DBDatetime(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DBTimestamp(pub i64);

#[derive(Debug, Clone, PartialEq, Default)]
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

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

// ============================================================================
// Implementations for Value
// ============================================================================
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(DBInt(v))
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Int(DBInt(v as i64))
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::UInt(DBUInt(v))
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::UInt(DBUInt(v as u64))
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(DBFloat(v))
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(DBFloat(v as f64))
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(DBBoolean(v))
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(DBText(v))
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(DBText(v.to_string()))
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Blob(DBBlob(v))
    }
}

// ============================================================================
// From Implementations for DBInt
// ============================================================================

impl From<i8> for DBInt {
    fn from(v: i8) -> Self {
        DBInt(v as i64)
    }
}

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

impl From<isize> for DBInt {
    fn from(v: isize) -> Self {
        DBInt(v as i64)
    }
}

// ============================================================================
// From Implementations for DBUInt
// ============================================================================

impl From<u8> for DBUInt {
    fn from(v: u8) -> Self {
        DBUInt(v as u64)
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

impl From<usize> for DBUInt {
    fn from(v: usize) -> Self {
        DBUInt(v as u64)
    }
}

// ============================================================================
// From Implementations for DBFloat
// ============================================================================

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

// ============================================================================
// From Implementations for DBBoolean
// ============================================================================

impl From<bool> for DBBoolean {
    fn from(v: bool) -> Self {
        DBBoolean(v)
    }
}

// ============================================================================
// From Implementations for DBText
// ============================================================================

impl From<String> for DBText {
    fn from(v: String) -> Self {
        DBText(v)
    }
}

impl From<&str> for DBText {
    fn from(v: &str) -> Self {
        DBText(v.to_string())
    }
}

impl From<&String> for DBText {
    fn from(v: &String) -> Self {
        DBText(v.clone())
    }
}

// ============================================================================
// From Implementations for DBVarChar
// ============================================================================

impl From<String> for DBVarChar {
    fn from(v: String) -> Self {
        DBVarChar::new(v, 255).unwrap_or_else(|_| DBVarChar {
            value: String::new(),
            max_length: 255,
        })
    }
}

impl From<&str> for DBVarChar {
    fn from(v: &str) -> Self {
        DBVarChar::new(v.to_string(), 255).unwrap_or_else(|_| DBVarChar {
            value: String::new(),
            max_length: 255,
        })
    }
}

impl From<&String> for DBVarChar {
    fn from(v: &String) -> Self {
        DBVarChar::new(v.clone(), 255).unwrap_or_else(|_| DBVarChar {
            value: String::new(),
            max_length: 255,
        })
    }
}

// ============================================================================
// From Implementations for DBDate
// ============================================================================

impl From<String> for DBDate {
    fn from(v: String) -> Self {
        DBDate(v)
    }
}

impl From<&str> for DBDate {
    fn from(v: &str) -> Self {
        DBDate(v.to_string())
    }
}

impl From<&String> for DBDate {
    fn from(v: &String) -> Self {
        DBDate(v.clone())
    }
}

// ============================================================================
// From Implementations for DBDatetime
// ============================================================================

impl From<String> for DBDatetime {
    fn from(v: String) -> Self {
        DBDatetime(v)
    }
}

impl From<&str> for DBDatetime {
    fn from(v: &str) -> Self {
        DBDatetime(v.to_string())
    }
}

impl From<&String> for DBDatetime {
    fn from(v: &String) -> Self {
        DBDatetime(v.clone())
    }
}

// ============================================================================
// From Implementations for DBTimestamp
// ============================================================================

impl From<i64> for DBTimestamp {
    fn from(v: i64) -> Self {
        DBTimestamp(v)
    }
}

impl From<i32> for DBTimestamp {
    fn from(v: i32) -> Self {
        DBTimestamp(v as i64)
    }
}

// ============================================================================
// From Implementations for DBBlob
// ============================================================================

impl From<Vec<u8>> for DBBlob {
    fn from(v: Vec<u8>) -> Self {
        DBBlob(v)
    }
}

impl From<&[u8]> for DBBlob {
    fn from(v: &[u8]) -> Self {
        DBBlob(v.to_vec())
    }
}

impl From<&Vec<u8>> for DBBlob {
    fn from(v: &Vec<u8>) -> Self {
        DBBlob(v.clone())
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn wrong_type(expected: &str, value: &Value) -> DbError {
    DbError::InvalidData(format!(
        "Cannot convert {:?} to {}",
        value, expected
    ))
}

// ============================================================================
// DbType Trait
// ============================================================================

pub trait DbType: Sized {
    fn column_type() -> ColumnType;
    fn to_value(self) -> Value;
    fn from_value(value: &Value) -> DbResult<Self>;
}

macro_rules! impl_simple_dbtype {
    ($type:ty, $variant:ident, $col:expr) => {
        impl DbType for $type {
            fn column_type() -> ColumnType {
                $col
            }

            fn to_value(self) -> Value {
                Value::$variant(self)
            }

            fn from_value(value: &Value) -> DbResult<Self> {
                if let Value::$variant(v) = value {
                    Ok(v.clone())
                } else {
                    Err(wrong_type(stringify!($type), value))
                }
            }
        }

        impl From<$type> for Value {
            fn from(v: $type) -> Self {
                Value::$variant(v)
            }
        }
    };
}

impl_simple_dbtype!(DBInt, Int, ColumnType::Int);
impl_simple_dbtype!(DBUInt, UInt, ColumnType::UInt);
impl_simple_dbtype!(DBFloat, Float, ColumnType::Float);
impl_simple_dbtype!(DBBoolean, Boolean, ColumnType::Boolean);
impl_simple_dbtype!(DBText, Text, ColumnType::Text);
impl_simple_dbtype!(DBDate, Date, ColumnType::Date);
impl_simple_dbtype!(DBDatetime, DateTime, ColumnType::DateTime);
impl_simple_dbtype!(DBTimestamp, Timestamp, ColumnType::Timestamp);
impl_simple_dbtype!(DBBlob, Blob, ColumnType::Blob);

impl DbType for DBVarChar {
    fn column_type() -> ColumnType {
        ColumnType::VarChar(255)
    }

    fn to_value(self) -> Value {
        Value::VarChar(self)
    }

    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::VarChar(v) => Ok(v.clone()),
            Value::Text(t) => DBVarChar::new(t.0.clone(), 255),
            _ => Err(wrong_type("DBVarChar", value)),
        }
    }
}

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

// ============================================================================
// Implementations for DBVarChar
// ============================================================================

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

// ============================================================================
// Utility Methods
// ============================================================================

impl DBTimestamp {
    pub fn now() -> Self {
        DBTimestamp(Utc::now().timestamp())
    }

    pub fn get_now() -> Self {
        Self::now()
    }
}

impl DBDate {
    pub fn now() -> Self {
        DBDate(Utc::now().format("%Y-%m-%d").to_string())
    }

    pub fn get_now() -> Self {
        Self::now()
    }
}

impl DBDatetime {
    pub fn now() -> Self {
        DBDatetime(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn get_now() -> Self {
        Self::now()
    }
}

// ============================================================================
// Utils
// ============================================================================

impl DBText {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl DBDate {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl DBDatetime {
    pub fn value(&self) -> &str {
        &self.0
    }
}