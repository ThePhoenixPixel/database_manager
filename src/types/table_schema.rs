use crate::types::column::Column;
use crate::types::index::Index;
use crate::types::foreign_key::ForeignKey;

#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
    pub foreign_keys: Vec<ForeignKey>,
}

impl TableSchema {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            columns: Vec::new(),
            indexes: Vec::new(),
            foreign_keys: Vec::new(),
        }
    }

    pub fn add_column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    pub fn add_index(mut self, index: Index) -> Self {
        self.indexes.push(index);
        self
    }

    pub fn add_foreign_key(mut self, fk: ForeignKey) -> Self {
        self.foreign_keys.push(fk);
        self
    }
}