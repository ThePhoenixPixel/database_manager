use async_trait::async_trait;

use crate::types::{DbResult, QueryFilters, Row, TableSchema, Value};


#[async_trait]
pub trait DatabaseController: Send + Sync {
    // Connection Management
    async fn connect(&mut self) -> DbResult<()>;
    async fn disconnect(&mut self) -> DbResult<()>;
    async fn is_connected(&self) -> bool;

    // Table Management
    async fn create_table(&self, schema: &TableSchema) -> DbResult<()>;
    async fn drop_table(&self, table_name: &str) -> DbResult<()>;
    async fn is_table_exists(&self, table_name: &str) -> DbResult<bool>;
    async fn list_tables(&self) -> DbResult<Vec<String>>;
    //async fn alter_table(&self, table_name: &str, operations: Vec<AlterTableOperation>) -> DbResult<()>;
    async fn get_table_schema(&self, table_name: &str) -> DbResult<TableSchema>;

    // CRUD Operations
    async fn insert(&self, table: &str, data: &Row) -> DbResult<Value>; // Returns inserted ID
    //async fn insert_many(&self, table: &str, data: &[Row]) -> DbResult<usize>; // Returns count
    async fn query(&self, table: &str, filters: &QueryFilters) -> DbResult<Vec<Row>>;
    async fn query_one(&self, table: &str, filters: &QueryFilters) -> DbResult<Option<Row>>;

    async fn query_with_join(&self, table: &str, joins: Vec<(&str, String, String)>, filters: &QueryFilters) -> DbResult<Vec<Row>>;

    async fn update(&self, table: &str, filters: &QueryFilters, data: &Row) -> DbResult<usize>; // Returns affected rows
    async fn delete(&self, table: &str, filters: &QueryFilters) -> DbResult<usize>; // Returns affected rows
    async fn count(&self, table: &str, filters: &QueryFilters) -> DbResult<usize>;

}