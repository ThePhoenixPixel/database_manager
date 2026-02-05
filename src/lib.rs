pub mod config;
pub mod drivers;
pub mod types;
pub mod controller;
pub mod manager;
pub mod table;


// Re-export commonly used types
pub use controller::DatabaseController;
pub use manager::DatabaseManager;
pub use table::Table;

// Re-export driver configs when features are enabled
#[cfg(feature = "sqlite")]
pub use drivers::sqlite::{DBSqliteConfig, SqliteManager};

#[cfg(feature = "mysql")]
pub use drivers::mysql::{DBMysqlConfig, MysqlManager};

// Re-export derive macro when available (this adds the #[derive(Table)] capability)
// The derive macro is automatically available when you use #[derive(Table)]
#[cfg(feature = "derive")]
pub use database_manager_derive::Table as TableDerive;
