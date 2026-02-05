#[derive(Debug, Clone, PartialEq)]
pub enum IndexType {
    Primary,
    Unique,
    Index,
    FullText,
}

#[derive(Debug, Clone)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub index_type: IndexType,
}

impl Index {
    pub fn new(name: impl Into<String>, columns: Vec<String>) -> Self {
        Self {
            name: name.into(),
            columns,
            index_type: IndexType::Index,
        }
    }

    pub fn unique(name: impl Into<String>, columns: Vec<String>) -> Self {
        Self {
            name: name.into(),
            columns,
            index_type: IndexType::Unique,
        }
    }

    pub fn primary(name: impl Into<String>, columns: Vec<String>) -> Self {
        Self {
            name: name.into(),
            columns,
            index_type: IndexType::Primary,
        }
    }

    pub fn fulltext(name: impl Into<String>, columns: Vec<String>) -> Self {
        Self {
            name: name.into(),
            columns,
            index_type: IndexType::FullText,
        }
    }

    pub fn index_type(mut self, index_type: IndexType) -> Self {
        self.index_type = index_type;
        self
    }
}
