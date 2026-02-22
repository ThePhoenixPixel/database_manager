use crate::types::column::Column;
use crate::types::foreign_key::ForeignKey;
use crate::types::index::Index;

#[derive(Debug, Clone)]
pub enum AlterTableOperation {
    AddColumn(Column),
    DropColumn(String),
    RenameColumn {
        old_name: String,
        new_name: String,
    },
    ModifyColumn(Column),
    AddIndex(Index),
    DropIndex(String),
    AddForeignKey(ForeignKey),
    DropForeignKey(String),
    RenameTable(String),
}

impl AlterTableOperation {
    pub fn add_column(column: Column) -> Self {
        Self::AddColumn(column)
    }

    pub fn drop_column(name: impl Into<String>) -> Self {
        Self::DropColumn(name.into())
    }

    pub fn rename_column(old_name: impl Into<String>, new_name: impl Into<String>) -> Self {
        Self::RenameColumn {
            old_name: old_name.into(),
            new_name: new_name.into(),
        }
    }

    pub fn modify_column(column: Column) -> Self {
        Self::ModifyColumn(column)
    }

    pub fn add_index(index: Index) -> Self {
        Self::AddIndex(index)
    }

    pub fn drop_index(name: impl Into<String>) -> Self {
        Self::DropIndex(name.into())
    }

    pub fn add_foreign_key(foreign_key: ForeignKey) -> Self {
        Self::AddForeignKey(foreign_key)
    }

    pub fn drop_foreign_key(name: impl Into<String>) -> Self {
        Self::DropForeignKey(name.into())
    }

    pub fn rename_table(new_name: impl Into<String>) -> Self {
        Self::RenameTable(new_name.into())
    }
}
