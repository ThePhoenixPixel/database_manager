use async_trait::async_trait;

use crate::config::DatabaseConfig;
use crate::controller::DatabaseController;
use crate::types::{Row, Value, DbResult, TableSchema, QueryFilters};


#[cfg(not(any(feature = "sqlite", feature = "mysql")))]
use crate::types::DbError;

/// Main DatabaseManager wrapper that provides a unified interface
/// regardless of the underlying database driver
pub struct DatabaseManager {
    manager: Box<dyn DatabaseController>,
}

impl DatabaseManager {
    /// Create a new DatabaseManager instance from a config
    pub fn new(config: DatabaseConfig) -> DbResult<Self> {
        let manager: Box<dyn DatabaseController> = match config {
            #[cfg(feature = "sqlite")]
            DatabaseConfig::Sqlite(cfg) => {
                use crate::drivers::sqlite::SqliteManager;
                Box::new(SqliteManager::new(cfg))
            }
            #[cfg(feature = "mysql")]
            DatabaseConfig::Mysql(cfg) => {
                use crate::drivers::mysql::MysqlManager;
                Box::new(MysqlManager::new(cfg))
            }
            #[cfg(not(any(feature = "sqlite", feature = "mysql")))]
            _ => {
                return Err(DbError::ConnectionError(
                    "No database driver feature enabled".to_string()
                ))
            }
        };

        Ok(Self { manager })
    }

    /// Connect to the database
    pub async fn connect(&mut self) -> DbResult<()> {
        self.manager.connect().await
    }

    /// Disconnect from the database
    pub async fn disconnect(&mut self) -> DbResult<()> {
        self.manager.disconnect().await
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        self.manager.is_connected().await
    }

    /// Create a table
    pub async fn create_table(&self, schema: &TableSchema) -> DbResult<()> {
        self.manager.create_table(schema).await
    }

    /// Drop a table
    pub async fn drop_table(&self, table_name: &str) -> DbResult<()> {
        self.manager.drop_table(table_name).await
    }

    /// Check if table exists
    pub async fn is_table_exists(&self, table_name: &str) -> DbResult<bool> {
        self.manager.is_table_exists(table_name).await
    }

    /// List all tables
    pub async fn list_tables(&self) -> DbResult<Vec<String>> {
        self.manager.list_tables().await
    }

    /// Get table schema
    pub async fn get_table_schema(&self, table_name: &str) -> DbResult<TableSchema> {
        self.manager.get_table_schema(table_name).await
    }

    /// Insert a row
    pub async fn insert(&self, table: &str, data: &Row) -> DbResult<Value> {
        self.manager.insert(table, data).await
    }

    /// Query rows
    pub async fn query(&self, table: &str, filters: &QueryFilters) -> DbResult<Vec<Row>> {
        self.manager.query(table, filters).await
    }

    /// Query one row
    pub async fn query_one(&self, table: &str, filters: &QueryFilters) -> DbResult<Option<Row>> {
        self.manager.query_one(table, filters).await
    }

    /// Update rows
    pub async fn update(&self, table: &str, filters: &QueryFilters, data: &Row) -> DbResult<usize> {
        self.manager.update(table, filters, data).await
    }

    /// Delete rows
    pub async fn delete(&self, table: &str, filters: &QueryFilters) -> DbResult<usize> {
        self.manager.delete(table, filters).await
    }

    /// Count rows
    pub async fn count(&self, table: &str, filters: &QueryFilters) -> DbResult<usize> {
        self.manager.count(table, filters).await
    }
}

// Optional: Implement DatabaseController trait for DatabaseManager to allow it to be used
// as a DatabaseController itself
#[async_trait]
impl DatabaseController for DatabaseManager {
    async fn connect(&mut self) -> DbResult<()> {
        self.connect().await
    }

    async fn disconnect(&mut self) -> DbResult<()> {
        self.disconnect().await
    }

    async fn is_connected(&self) -> bool {
        self.is_connected().await
    }

    async fn create_table(&self, schema: &TableSchema) -> DbResult<()> {
        self.create_table(schema).await
    }

    async fn drop_table(&self, table_name: &str) -> DbResult<()> {
        self.drop_table(table_name).await
    }

    async fn is_table_exists(&self, table_name: &str) -> DbResult<bool> {
        self.is_table_exists(table_name).await
    }

    async fn list_tables(&self) -> DbResult<Vec<String>> {
        self.list_tables().await
    }

    async fn get_table_schema(&self, table_name: &str) -> DbResult<TableSchema> {
        self.get_table_schema(table_name).await
    }

    async fn insert(&self, table: &str, data: &Row) -> DbResult<Value> {
        self.insert(table, data).await
    }

    async fn query(&self, table: &str, filters: &QueryFilters) -> DbResult<Vec<Row>> {
        self.query(table, filters).await
    }

    async fn query_one(&self, table: &str, filters: &QueryFilters) -> DbResult<Option<Row>> {
        self.query_one(table, filters).await
    }

    async fn query_with_join(&self, table: &str, joins: Vec<(&str, &str, &str)>, filters: &QueryFilters) -> DbResult<Vec<Row>> {
        self.query_with_join(table, joins, filters).await
    }

    async fn update(&self, table: &str, filters: &QueryFilters, data: &Row) -> DbResult<usize> {
        self.update(table, filters, data).await
    }

    async fn delete(&self, table: &str, filters: &QueryFilters) -> DbResult<usize> {
        self.delete(table, filters).await
    }

    async fn count(&self, table: &str, filters: &QueryFilters) -> DbResult<usize> {
        self.count(table, filters).await
    }
}
