# Database Manager

A flexible, type-safe ORM for Rust with support for SQLite and MySQL/MariaDB.

## Features

- 🔌 **Multi-Driver Support**: SQLite and MySQL/MariaDB via feature flags
- 🎯 **Type-Safe**: Fully typed value system aligned with column types
- 🚀 **Async/Await**: Fully asynchronous with tokio
- 🔧 **Schema Management**: Table schema definition with builder pattern
- 🔑 **Foreign Keys & Indexes**: Complete support for relations
- 📦 **Simple API**: Unified interface for all databases

## Installation

Add the package to your `Cargo.toml`:

```toml
[dependencies]
database_manager = { path = ".", features = ["sqlite"] }

# Or with MySQL
database_manager = { path = ".", features = ["mysql"] }

# Or with both
database_manager = { path = ".", features = ["all-drivers"] }
```

## Quick Start

### SQLite Example

```rust
use database_manager::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> DbResult<()> {
    // Create SQLite configuration
    let config = DatabaseConfig::Sqlite(DBSqliteConfig::new("test.db"));

    // Create DatabaseManager instance
    let mut db = DatabaseManager::new(config)?;

    // Connect to database
    db.connect().await?;

    // Define table schema
    let schema = TableSchema::new("users")
        .add_column(
            Column::new("id", ColumnType::Int)
                .primary_key()
                .auto_increment()
        )
        .add_column(
            Column::new("name", ColumnType::VarChar(255))
        )
        .add_column(
            Column::new("email", ColumnType::VarChar(255))
                .unique()
        )
        .add_column(
            Column::new("age", ColumnType::Int)
        )
        .add_column(
            Column::new("active", ColumnType::Boolean)
        );

    // Create table
    db.create_table(&schema).await?;

    // Insert data
    let mut row = HashMap::new();
    row.insert("name".to_string(), Value::Text(DBText("Alice".to_string())));
    row.insert("email".to_string(), Value::Text(DBText("alice@example.com".to_string())));
    row.insert("age".to_string(), Value::Int(DBInt(30)));
    row.insert("active".to_string(), Value::Boolean(DBBoolean(true)));

    let id = db.insert("users", &row).await?;
    println!("Inserted user with ID: {:?}", id);

    // Query all users
    let users = db.query("users", &QueryFilters::new()).await?;
    println!("\nAll users:");
    for user in users {
        println!("{:?}", user);
    }

    // Query with filters
    let filters = QueryFilters::new()
        .add_filter(Filter::eq("age", Value::Int(DBInt(30))))
        .order_by("name", OrderDirection::Asc)
        .limit(10);

    let filtered_users = db.query("users", &filters).await?;
    println!("\nFiltered users:");
    for user in filtered_users {
        println!("{:?}", user);
    }

    // Update data
    let mut update_data = HashMap::new();
    update_data.insert("age".to_string(), Value::Int(DBInt(31)));

    let update_filters = QueryFilters::new()
        .add_filter(Filter::eq("email", Value::Text(DBText("alice@example.com".to_string()))));

    let updated = db.update("users", &update_filters, &update_data).await?;
    println!("\nUpdated {} rows", updated);

    // Count users
    let count = db.count("users", &QueryFilters::new()).await?;
    println!("Total users: {}", count);

    // Disconnect
    db.disconnect().await?;

    Ok(())
}
```

### MySQL Example

```rust
use database_manager::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> DbResult<()> {
    // Create MySQL configuration
    let host = Authority::parse("localhost:3306")
        .expect("Invalid host");

    let config = DatabaseConfig::Mysql(
        DBMysqlConfig::new(
            host,
            "root".to_string(),
            "password".to_string(),
            "test_db".to_string(),
            5 // pool size
        )
    );

    // Create DatabaseManager instance
    let mut db = DatabaseManager::new(config)?;

    // Connect to database
    db.connect().await?;

    // Define products table schema
    let products_schema = TableSchema::new("products")
        .add_column(
            Column::new("id", ColumnType::Int)
                .primary_key()
                .auto_increment()
        )
        .add_column(
            Column::new("name", ColumnType::VarChar(255))
        )
        .add_column(
            Column::new("price", ColumnType::Float)
        )
        .add_column(
            Column::new("stock", ColumnType::UInt)
        )
        .add_column(
            Column::new("description", ColumnType::Text)
                .nullable()
        )
        .add_column(
            Column::new("created_at", ColumnType::DateTime)
        )
        .add_index(
            Index::unique("idx_name", vec!["name".to_string()])
        );

    // Create table
    db.create_table(&products_schema).await?;

    // Insert multiple products
    let products = vec![
        ("Laptop", 999.99, 10, "High-performance laptop"),
        ("Mouse", 29.99, 100, "Wireless gaming mouse"),
        ("Keyboard", 79.99, 50, "Mechanical keyboard"),
    ];

    for (name, price, stock, desc) in products {
        let mut row = HashMap::new();
        row.insert("name".to_string(), Value::VarChar(DBVarChar::new(name.to_string(), 255)?));
        row.insert("price".to_string(), Value::Float(DBFloat(price)));
        row.insert("stock".to_string(), Value::UInt(DBUInt(stock)));
        row.insert("description".to_string(), Value::Text(DBText(desc.to_string())));
        row.insert("created_at".to_string(), Value::DateTime(DBDatetime("2024-01-01 12:00:00".to_string())));

        let id = db.insert("products", &row).await?;
        println!("Inserted product with ID: {:?}", id);
    }

    // Query products with price > 50
    let filters = QueryFilters::new()
        .add_filter(Filter::gt("price", Value::Float(DBFloat(50.0))))
        .order_by("price", OrderDirection::Desc);

    let expensive_products = db.query("products", &filters).await?;
    println!("\nProducts over $50:");
    for product in expensive_products {
        if let Some(name) = product.get("name") {
            if let Some(price) = product.get("price") {
                println!("- {:?}: {:?}", name, price);
            }
        }
    }

    // Update stock
    let mut update_data = HashMap::new();
    update_data.insert("stock".to_string(), Value::UInt(DBUInt(95)));

    let update_filters = QueryFilters::new()
        .add_filter(Filter::eq("name", Value::VarChar(DBVarChar::new("Mouse".to_string(), 255)?)));

    let updated = db.update("products", &update_filters, &update_data).await?;
    println!("\nUpdated stock for {} products", updated);

    // Get single product
    let filters = QueryFilters::new()
        .add_filter(Filter::eq("name", Value::VarChar(DBVarChar::new("Laptop".to_string(), 255)?)));

    if let Some(laptop) = db.query_one("products", &filters).await? {
        println!("\nFound laptop: {:?}", laptop);
    }

    // Delete product
    let delete_filters = QueryFilters::new()
        .add_filter(Filter::eq("name", Value::VarChar(DBVarChar::new("Keyboard".to_string(), 255)?)));

    let deleted = db.delete("products", &delete_filters).await?;
    println!("\nDeleted {} products", deleted);

    // List all tables
    let tables = db.list_tables().await?;
    println!("\nTables in database: {:?}", tables);

    // Get table schema
    let schema = db.get_table_schema("products").await?;
    println!("\nProducts table schema:");
    for column in &schema.columns {
        println!("  - {}: {:?}", column.name, column.column_type);
    }

    // Disconnect
    db.disconnect().await?;

    Ok(())
}
```

## Data Types

### Value Types

The `Value` enum is fully synchronized with `ColumnType`:

```rust
// Integer types
Value::Int(DBInt(i64))          // ColumnType::Int
Value::UInt(DBUInt(u64))        // ColumnType::UInt

// Float types
Value::Float(DBFloat(f64))      // ColumnType::Float

// Text types
Value::Text(DBText(String))             // ColumnType::Text
Value::VarChar(DBVarChar)               // ColumnType::VarChar(n)

// Other types
Value::Boolean(DBBoolean(bool))         // ColumnType::Boolean
Value::Date(DBDate(String))             // ColumnType::Date
Value::DateTime(DBDatetime(String))     // ColumnType::DateTime
Value::Timestamp(DBTimestamp(i64))      // ColumnType::Timestamp
Value::Blob(DBBlob(Vec<u8>))            // ColumnType::Blob
Value::Null                             // NULL value
```

### VarChar Usage

VarChar has length validation:

```rust
// Create with validation
let name = DBVarChar::new("John".to_string(), 50)?; // OK
let too_long = DBVarChar::new("very long text...".to_string(), 5)?; // Error

// Access value
let value = name.value(); // Returns &str
let max_len = name.max_length(); // Returns usize
```

## Query Filters

Build complex queries with filters:

```rust
use database_manager::*;

let filters = QueryFilters::new()
    .add_filter(Filter::eq("age", Value::Int(DBInt(25))))
    .add_filter(Filter::gt("score", Value::Int(DBInt(100))))
    .add_filter(Filter::like("name", Value::Text(DBText("%John%".to_string()))))
    .order_by("created_at", OrderDirection::Desc)
    .limit(20)
    .offset(0);

let results = db.query("users", &filters).await?;
```

### Available Filter Operators

```rust
// Comparison operators
Filter::eq(column, value)           // column = value
Filter::not_eq(column, value)       // column != value
Filter::gt(column, value)           // column > value
Filter::lt(column, value)           // column < value
Filter::gte(column, value)          // column >= value
Filter::lte(column, value)          // column <= value

// Pattern matching
Filter::like(column, pattern)       // column LIKE pattern

// NULL checks
Filter::is_null(column)             // column IS NULL
Filter::is_not_null(column)         // column IS NOT NULL

// List operations
Filter::in_values(column, values)   // column IN (values)
Filter::between(column, val1, val2) // column BETWEEN val1 AND val2
```

## Schema Management

### Defining Tables

```rust
let schema = TableSchema::new("posts")
    .add_column(
        Column::new("id", ColumnType::Int)
            .primary_key()
            .auto_increment()
    )
    .add_column(
        Column::new("user_id", ColumnType::Int)
    )
    .add_column(
        Column::new("title", ColumnType::VarChar(255))
    )
    .add_column(
        Column::new("content", ColumnType::Text)
    )
    .add_column(
        Column::new("published", ColumnType::Boolean)
            .default_value(Value::Boolean(DBBoolean(false)))
    );
```

### Foreign Keys

```rust
let schema = TableSchema::new("posts")
    .add_column(Column::new("id", ColumnType::Int).primary_key())
    .add_column(Column::new("user_id", ColumnType::Int))
    .add_foreign_key(
        ForeignKey::new(
            "fk_user_posts",
            vec!["user_id".to_string()],
            "users",
            vec!["id".to_string()]
        )
        .on_delete(ForeignKeyAction::Cascade)
        .on_update(ForeignKeyAction::Cascade)
    );
```

### Foreign Key Actions

```rust
ForeignKeyAction::Cascade       // CASCADE
ForeignKeyAction::SetNull       // SET NULL
ForeignKeyAction::SetDefault    // SET DEFAULT
ForeignKeyAction::Restrict      // RESTRICT
ForeignKeyAction::NoAction      // NO ACTION
```

### Indexes

```rust
let schema = TableSchema::new("users")
    .add_column(Column::new("email", ColumnType::VarChar(255)))
    .add_column(Column::new("username", ColumnType::VarChar(100)))
    .add_index(
        Index::unique("idx_email", vec!["email".to_string()])
    )
    .add_index(
        Index::new("idx_username", vec!["username".to_string()])
            .index_type(IndexType::BTree)
    );
```

### Index Types

```rust
IndexType::Primary      // Primary key index
IndexType::Unique       // Unique index
IndexType::BTree        // B-Tree index (default)
IndexType::Hash         // Hash index
IndexType::FullText     // Full-text index (MySQL only)
```

## CRUD Operations

### Insert

```rust
let mut row = HashMap::new();
row.insert("name".to_string(), Value::Text(DBText("Alice".to_string())));
row.insert("age".to_string(), Value::Int(DBInt(30)));

let id = db.insert("users", &row).await?;
println!("Inserted ID: {:?}", id);
```

### Query

```rust
// Get all records
let all_users = db.query("users", &QueryFilters::new()).await?;

// Get with filters
let filters = QueryFilters::new()
    .add_filter(Filter::gte("age", Value::Int(DBInt(18))))
    .limit(10);

let adults = db.query("users", &filters).await?;

// Get single record
let filters = QueryFilters::new()
    .add_filter(Filter::eq("id", Value::Int(DBInt(1))));

let user = db.query_one("users", &filters).await?;
```

### Update

```rust
let mut update_data = HashMap::new();
update_data.insert("age".to_string(), Value::Int(DBInt(31)));

let filters = QueryFilters::new()
    .add_filter(Filter::eq("id", Value::Int(DBInt(1))));

let updated_count = db.update("users", &filters, &update_data).await?;
println!("Updated {} rows", updated_count);
```

### Delete

```rust
let filters = QueryFilters::new()
    .add_filter(Filter::lt("age", Value::Int(DBInt(18))));

let deleted_count = db.delete("users", &filters).await?;
println!("Deleted {} rows", deleted_count);
```

### Count

```rust
let filters = QueryFilters::new()
    .add_filter(Filter::eq("active", Value::Boolean(DBBoolean(true))));

let active_count = db.count("users", &filters).await?;
println!("Active users: {}", active_count);
```

## Table Operations

```rust
// Check if table exists
let exists = db.is_table_exists("users").await?;
if !exists {
    db.create_table(&schema).await?;
}

// List all tables
let tables = db.list_tables().await?;
for table in tables {
    println!("Table: {}", table);
}

// Get table schema
let schema = db.get_table_schema("users").await?;
println!("Columns:");
for column in &schema.columns {
    println!("  - {}: {:?}", column.name, column.column_type);
}

// Drop table
db.drop_table("old_table").await?;
```

## Configuration

### SQLite Configuration

```rust
use database_manager::*;

// Simple path
let config = DatabaseConfig::Sqlite(
    DBSqliteConfig::new("database.db")
);

// In-memory database
let config = DatabaseConfig::Sqlite(
    DBSqliteConfig::new(":memory:")
);
```

### MySQL Configuration

```rust
use database_manager::*;
use address::Authority;

let host = Authority::parse("localhost:3306")?;

let config = DatabaseConfig::Mysql(
    DBMysqlConfig::new(
        host,
        "username".to_string(),
        "password".to_string(),
        "database_name".to_string(),
        10 // connection pool size
    )
);
```

## Feature Flags

Control which database drivers are compiled:

```toml
[dependencies]
database_manager = { path = ".", features = ["sqlite"] }
```

### Available Features

- `sqlite` - Enable SQLite support
- `mysql` - Enable MySQL/MariaDB support
- `all-drivers` - Enable all database drivers
- `serde` - Enable serde support (enabled by default)

### Examples

```toml
# SQLite only
database_manager = { path = ".", features = ["sqlite"] }

# MySQL only
database_manager = { path = ".", features = ["mysql"] }

# Both drivers
database_manager = { path = ".", features = ["all-drivers"] }

# SQLite without serde
database_manager = { path = ".", features = ["sqlite"], default-features = false }
```

## Running Examples

Complete examples are available in the `examples/` directory:

```bash
# SQLite example
cargo run --example sqlite_example --features sqlite

# MySQL example
cargo run --example mysql_example --features mysql

# Run with both drivers
cargo run --example sqlite_example --features all-drivers
```

## Error Handling

All database operations return `DbResult<T>` which is an alias for `Result<T, DbError>`:

```rust
use database_manager::*;

match db.insert("users", &row).await {
    Ok(id) => println!("Success: {:?}", id),
    Err(DbError::ConnectionError(msg)) => eprintln!("Connection failed: {}", msg),
    Err(DbError::InsertError { table, message }) => {
        eprintln!("Insert into {} failed: {}", table, message)
    }
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### Error Types

```rust
DbError::ConnectionError(String)
DbError::CreateTableError { table: String, message: String }
DbError::DropTableError { table: String, message: String }
DbError::InsertError { table: String, message: String }
DbError::QueryError { table: Option<String>, message: String }
DbError::UpdateError { table: String, message: String }
DbError::DeleteError { table: String, message: String }
DbError::SchemaError { table: Option<String>, message: String }
DbError::InvalidData(String)
DbError::NotSupported(String)
```

## Database-Specific Notes

### SQLite

- VARCHAR lengths are not enforced (stored as TEXT)
- UInt values are stored as signed INTEGER (i64)
- Boolean values are stored as INTEGER (0/1)
- Timestamps are stored as INTEGER (Unix timestamp)

### MySQL/MariaDB

- Full VARCHAR length enforcement
- Native UNSIGNED INTEGER support
- Native BOOLEAN type
- Native TIMESTAMP type with timezone support

## Best Practices

1. **Always use prepared statements** (done automatically by the library)
2. **Close connections** when done:
   ```rust
   db.disconnect().await?;
   ```
3. **Use VarChar for limited strings**, Text for large content
4. **Validate data** before insertion when using VarChar
5. **Use transactions** for multiple related operations (coming soon)

## Roadmap

- [ ] Transaction support
- [ ] Connection pooling improvements
- [ ] PostgreSQL support
- [ ] Query builder improvements
- [ ] Migration system
- [ ] Derive macros for models
