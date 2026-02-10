use async_trait::async_trait;

use crate::types::{TableSchema, DbResult, Row, Value, QueryFilters};
use crate::controller::DatabaseController;

/// Trait for database table definitions
#[async_trait]
pub trait Table: Sized {
    /// Returns the name of the table
    fn table_name() -> &'static str;

    /// Returns the table schema
    fn table_schema() -> TableSchema;

    /// Converts the struct instance to a Row (HashMap)
    fn to_row(&self) -> Row;

    /// Converts a Row (HashMap) to a struct instance
    fn from_row(row: &Row) -> DbResult<Self>;

    /// Synchronizes the table with the database (creates if not exists)
    async fn sync<M: DatabaseController>(manager: &M) -> DbResult<()> {
        let table_name = Self::table_name();
        let schema = Self::table_schema();

        if !manager.is_table_exists(table_name).await? {
            manager.create_table(&schema).await?;
        }

        Ok(())
    }

    /// Drops the table from the database
    async fn drop<M: DatabaseController>(manager: &M) -> DbResult<()> {
        let table_name = Self::table_name();
        manager.drop_table(table_name).await
    }

    /// Inserts this struct instance into the database
    async fn insert<M: DatabaseController>(&self, manager: &M) -> DbResult<Value> {
        let table_name = Self::table_name();
        let row = self.to_row();
        manager.insert(table_name, &row).await
    }

    /// Query all records and convert to Vec<Self>
    async fn all<M: DatabaseController>(manager: &M) -> DbResult<Vec<Self>> {
        let rows = manager.query(Self::table_name(), &QueryFilters::new()).await?;
        rows.iter().map(|row| Self::from_row(row)).collect()
    }

    /// Query with filters and convert to Vec<Self>
    async fn find<M: DatabaseController>(manager: &M, filters: &QueryFilters) -> DbResult<Vec<Self>> {
        let rows = manager.query(Self::table_name(), filters).await?;
        rows.iter().map(|row| Self::from_row(row)).collect()
    }

    /// Query one record and convert to Option<Self>
    async fn find_one<M: DatabaseController>(manager: &M, filters: &QueryFilters) -> DbResult<Option<Self>> {
        let row_opt = manager.query_one(Self::table_name(), filters).await?;
        match row_opt {
            Some(row) => Ok(Some(Self::from_row(&row)?)),
            None => Ok(None),
        }
    }
}
