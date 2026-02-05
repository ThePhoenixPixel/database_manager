
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    SmallInt(i16),
    Integer(i32),
    BigInt(i64),
    Float(f32),
    Double(f64),
    Text(String),
    VarChar(String),
    Boolean(bool),
    Date(String),        // Format: yyyy-mm-dd
    DateTime(String),    // Format: yyyy-mm-dd hh:mm:ss
    Timestamp(String),   // Format: yyyy-mm-dd hh:mm:ss
    Json(String),
    Blob(Vec<u8>),
}

// Type aliases for convenience
pub type DBSmallInt = i16;
pub type DBInteger = i32;
pub type DBBigInt = i64;
pub type DBFloat = f32;
pub type DBDouble = f64;
pub type DBText = String;
pub type DBBoolean = bool;
pub type DBDatetime = String;

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_smallint(&self) -> Option<i16> {
        match self {
            Value::SmallInt(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i32> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_bigint(&self) -> Option<i64> {
        match self {
            Value::BigInt(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match self {
            Value::Double(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(s) | Value::VarChar(s) | Value::Json(s)
            | Value::Date(s) | Value::DateTime(s) | Value::Timestamp(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_date(&self) -> Option<&str> {
        match self {
            Value::Date(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_datetime(&self) -> Option<&str> {
        match self {
            Value::DateTime(dt) => Some(dt),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<&str> {
        match self {
            Value::Timestamp(ts) => Some(ts),
            _ => None,
        }
    }

    pub fn as_json(&self) -> Option<&str> {
        match self {
            Value::Json(j) => Some(j),
            _ => None,
        }
    }

    pub fn as_blob(&self) -> Option<&[u8]> {
        match self {
            Value::Blob(b) => Some(b),
            _ => None,
        }
    }

    // Convenience method to convert any number to i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::SmallInt(i) => Some(*i as i64),
            Value::Integer(i) => Some(*i as i64),
            Value::BigInt(i) => Some(*i),
            _ => None,
        }
    }

    // Convenience method to convert any float to f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f as f64),
            Value::Double(f) => Some(*f),
            _ => None,
        }
    }
}

// From implementations for common types
impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::SmallInt(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Integer(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::BigInt(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Text(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Text(value.to_string())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Blob(value)
    }
}
