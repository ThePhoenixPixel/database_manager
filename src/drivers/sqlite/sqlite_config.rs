use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DBSqliteConfig {
    path: String,
}

impl DBSqliteConfig {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

