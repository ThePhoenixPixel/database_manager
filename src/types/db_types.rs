use crate::types::{ColumnType, Value, DbResult, DbError};

/// Trait for types that can be mapped to database column types
pub trait DbType {
    fn column_type() -> ColumnType;
    fn to_value(&self) -> Value;
    fn from_value(value: &Value) -> DbResult<Self> where Self: Sized;
}

// Wrapper types for explicit column type mapping
#[derive(Debug, Clone)]
pub struct DbText(pub String);

#[derive(Debug, Clone)]
pub struct DbVarChar<const N: usize>(pub String);

#[derive(Debug, Clone)]
pub struct DbSmallInt(pub i16);

#[derive(Debug, Clone)]
pub struct DbInteger(pub i32);

#[derive(Debug, Clone)]
pub struct DbBigInt(pub i64);

#[derive(Debug, Clone)]
pub struct DbFloat(pub f32);

#[derive(Debug, Clone)]
pub struct DbDouble(pub f64);

#[derive(Debug, Clone)]
pub struct DbBoolean(pub bool);

#[derive(Debug, Clone)]
pub struct DbDate(pub String);

#[derive(Debug, Clone)]
pub struct DbDateTime(pub String);

#[derive(Debug, Clone)]
pub struct DbTimestamp(pub String);

#[derive(Debug, Clone)]
pub struct DbJson(pub String);

#[derive(Debug, Clone)]
pub struct DbBlob(pub Vec<u8>);

// Implement DbType for wrapper types
impl DbType for DbText {
    fn column_type() -> ColumnType {
        ColumnType::Text
    }
    fn to_value(&self) -> Value {
        Value::Text(self.0.clone())
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Text(s) => Ok(DbText(s.clone())),
            Value::VarChar(s) => Ok(DbText(s.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbText", value))),
        }
    }
}

impl<const N: usize> DbType for DbVarChar<N> {
    fn column_type() -> ColumnType {
        ColumnType::VarChar(N)
    }
    fn to_value(&self) -> Value {
        Value::VarChar(self.0.clone())
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::VarChar(s) => Ok(DbVarChar(s.clone())),
            Value::Text(s) => Ok(DbVarChar(s.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbVarChar", value))),
        }
    }
}

impl DbType for DbSmallInt {
    fn column_type() -> ColumnType {
        ColumnType::SmallInt
    }
    fn to_value(&self) -> Value {
        Value::SmallInt(self.0)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::SmallInt(i) => Ok(DbSmallInt(*i)),
            Value::Integer(i) => Ok(DbSmallInt(*i as i16)),
            Value::BigInt(i) => Ok(DbSmallInt(*i as i16)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbSmallInt", value))),
        }
    }
}

impl DbType for DbInteger {
    fn column_type() -> ColumnType {
        ColumnType::Integer
    }
    fn to_value(&self) -> Value {
        Value::Integer(self.0)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Integer(i) => Ok(DbInteger(*i)),
            Value::SmallInt(i) => Ok(DbInteger(*i as i32)),
            Value::BigInt(i) => Ok(DbInteger(*i as i32)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbInteger", value))),
        }
    }
}

impl DbType for DbBigInt {
    fn column_type() -> ColumnType {
        ColumnType::BigInt
    }
    fn to_value(&self) -> Value {
        Value::BigInt(self.0)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::BigInt(i) => Ok(DbBigInt(*i)),
            Value::Integer(i) => Ok(DbBigInt(*i as i64)),
            Value::SmallInt(i) => Ok(DbBigInt(*i as i64)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbBigInt", value))),
        }
    }
}

impl DbType for DbFloat {
    fn column_type() -> ColumnType {
        ColumnType::Float
    }
    fn to_value(&self) -> Value {
        Value::Float(self.0)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Float(f) => Ok(DbFloat(*f)),
            Value::Double(f) => Ok(DbFloat(*f as f32)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbFloat", value))),
        }
    }
}

impl DbType for DbDouble {
    fn column_type() -> ColumnType {
        ColumnType::Double
    }
    fn to_value(&self) -> Value {
        Value::Double(self.0)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Double(f) => Ok(DbDouble(*f)),
            Value::Float(f) => Ok(DbDouble(*f as f64)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbDouble", value))),
        }
    }
}

impl DbType for DbBoolean {
    fn column_type() -> ColumnType {
        ColumnType::Boolean
    }
    fn to_value(&self) -> Value {
        Value::Boolean(self.0)
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Boolean(b) => Ok(DbBoolean(*b)),
            Value::Integer(i) => Ok(DbBoolean(*i != 0)),
            Value::SmallInt(i) => Ok(DbBoolean(*i != 0)),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbBoolean", value))),
        }
    }
}

impl DbType for DbDate {
    fn column_type() -> ColumnType {
        ColumnType::Date
    }
    fn to_value(&self) -> Value {
        Value::Date(self.0.clone())
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Date(s) => Ok(DbDate(s.clone())),
            Value::Text(s) => Ok(DbDate(s.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbDate", value))),
        }
    }
}

impl DbType for DbDateTime {
    fn column_type() -> ColumnType {
        ColumnType::DateTime
    }
    fn to_value(&self) -> Value {
        Value::DateTime(self.0.clone())
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::DateTime(s) => Ok(DbDateTime(s.clone())),
            Value::Text(s) => Ok(DbDateTime(s.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbDateTime", value))),
        }
    }
}

impl DbType for DbTimestamp {
    fn column_type() -> ColumnType {
        ColumnType::Timestamp
    }
    fn to_value(&self) -> Value {
        Value::Timestamp(self.0.clone())
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Timestamp(s) => Ok(DbTimestamp(s.clone())),
            Value::Text(s) => Ok(DbTimestamp(s.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbTimestamp", value))),
        }
    }
}

impl DbType for DbJson {
    fn column_type() -> ColumnType {
        ColumnType::Json
    }
    fn to_value(&self) -> Value {
        Value::Json(self.0.clone())
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Json(s) => Ok(DbJson(s.clone())),
            Value::Text(s) => Ok(DbJson(s.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbJson", value))),
        }
    }
}

impl DbType for DbBlob {
    fn column_type() -> ColumnType {
        ColumnType::Blob
    }
    fn to_value(&self) -> Value {
        Value::Blob(self.0.clone())
    }
    fn from_value(value: &Value) -> DbResult<Self> {
        match value {
            Value::Blob(b) => Ok(DbBlob(b.clone())),
            _ => Err(DbError::InvalidData(format!("Cannot convert {:?} to DbBlob", value))),
        }
    }
}

// Implement DbType for Option<T> - makes column nullable
impl<T: DbType> DbType for Option<T> {
    fn column_type() -> ColumnType {
        T::column_type()
    }
    fn to_value(&self) -> Value {
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
