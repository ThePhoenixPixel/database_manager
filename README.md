# Database Manager

Ein flexibles, typsicheres ORM für Rust mit Unterstützung für SQLite und MySQL/MariaDB.

## Features

- 🔌 **Multi-Driver Support**: SQLite und MySQL/MariaDB über Feature-Flags
- 🎯 **Type-Safe**: Vollständig typisierte Value-Typen die mit ColumnTypes übereinstimmen
- 🚀 **Async/Await**: Vollständig asynchron mit tokio
- 🔧 **Schema Management**: Table-Schema-Definition mit Builder-Pattern
- 🔑 **Foreign Keys & Indexes**: Vollständige Unterstützung für Relationen
- 📦 **Einfache API**: Einheitliches Interface für alle Datenbanken

## Installation

Füge das Paket zu deiner `Cargo.toml` hinzu:

```toml
[dependencies]
database_manager = { path = ".", features = ["sqlite"] }

# Oder mit MySQL
database_manager = { path = ".", features = ["mysql"] }

# Oder mit beiden
database_manager = { path = ".", features = ["all-drivers"] }
```

## Quick Start

### SQLite

```rust
use database_manager::*;

#[tokio::main]
async fn main() -> DbResult<()> {
    // Config erstellen
    let config = DatabaseConfig::Sqlite(DBSqliteConfig::new("test.db"));

    // Database-Instance erstellen
    let mut db = Database::new(config)?;

    // Verbinden
    db.connect().await?;

    // Schema definieren
    let schema = TableSchema::new("users")
        .add_column(
            Column::new("id", ColumnType::BigInt)
                .primary_key()
                .auto_increment()
        )
        .add_column(
            Column::new("name", ColumnType::VarChar(255))
        )
        .add_column(
            Column::new("email", ColumnType::VarChar(255))
                .unique()
        );

    // Tabelle erstellen
    db.create_table(&schema).await?;

    // Daten einfügen
    let mut row = Row::new();
    row.insert("name".to_string(), Value::Text("Alice".to_string()));
    row.insert("email".to_string(), Value::Text("alice@example.com".to_string()));

    let id = db.insert("users", &row).await?;
    println!("Inserted with ID: {:?}", id);

    // Daten abfragen
    let users = db.query("users", &QueryFilters::new()).await?;

    for user in users {
        println!("{:?}", user);
    }

    db.disconnect().await?;
    Ok(())
}
```

### MySQL

```rust
use database_manager::*;
use address::Authority;

#[tokio::main]
async fn main() -> DbResult<()> {
    let host = Authority::parse("localhost:3306").unwrap();

    let config = DatabaseConfig::Mysql(
        DBMysqlConfig::new(
            host,
            "user".to_string(),
            "password".to_string(),
            "database".to_string(),
            5 // pool size
        )
    );

    let mut db = Database::new(config)?;
    db.connect().await?;

    // ... rest wie SQLite

    Ok(())
}
```

## Value-Typen

Die `Value`-Enum ist vollständig mit `ColumnType` synchronisiert:

```rust
// Integer-Typen
Value::SmallInt(i16)    // entspricht ColumnType::SmallInt
Value::Integer(i32)     // entspricht ColumnType::Integer
Value::BigInt(i64)      // entspricht ColumnType::BigInt

// Float-Typen
Value::Float(f32)       // entspricht ColumnType::Float
Value::Double(f64)      // entspricht ColumnType::Double

// Text-Typen
Value::Text(String)     // entspricht ColumnType::Text
Value::VarChar(String)  // entspricht ColumnType::VarChar(n)

// Andere Typen
Value::Boolean(bool)
Value::Date(String)
Value::DateTime(String)
Value::Timestamp(String)
Value::Json(String)
Value::Blob(Vec<u8>)
Value::Null
```

## Query Filters

```rust
use database_manager::*;

let filters = QueryFilters::new()
    .add_filter(Filter::eq("age", Value::Integer(25)))
    .add_filter(Filter::gt("score", Value::Integer(100)))
    .order_by("name", OrderDirection::Asc)
    .limit(10)
    .offset(0);

let results = db.query("users", &filters).await?;
```

### Verfügbare Filter-Operatoren

- `Filter::eq(column, value)` - Equals
- `Filter::not_eq(column, value)` - Not equals
- `Filter::gt(column, value)` - Greater than
- `Filter::lt(column, value)` - Less than
- `Filter::like(column, pattern)` - LIKE
- `Filter::is_null(column)` - IS NULL
- `Filter::in_values(column, values)` - IN

## Foreign Keys & Indexes

```rust
let schema = TableSchema::new("posts")
    .add_column(Column::new("id", ColumnType::BigInt).primary_key())
    .add_column(Column::new("user_id", ColumnType::BigInt))
    .add_column(Column::new("title", ColumnType::VarChar(255)))
    // Foreign Key
    .add_foreign_key(
        ForeignKey::new(
            "fk_user_posts",
            vec!["user_id".to_string()],
            "users",
            vec!["id".to_string()]
        )
        .cascade_delete()
        .cascade_update()
    )
    // Index
    .add_index(
        Index::unique("idx_title", vec!["title".to_string()])
    );
```

## CRUD Operations

```rust
// Insert
let id = db.insert("users", &row).await?;

// Query
let users = db.query("users", &filters).await?;
let user = db.query_one("users", &filters).await?;

// Update
let updated = db.update("users", &filters, &update_data).await?;

// Delete
let deleted = db.delete("users", &filters).await?;

// Count
let count = db.count("users", &filters).await?;
```

## Schema Management

```rust
// Check if table exists
if !db.is_table_exists("users").await? {
    db.create_table(&schema).await?;
}

// List tables
let tables = db.list_tables().await?;

// Get table schema
let schema = db.get_table_schema("users").await?;

// Drop table
db.drop_table("users").await?;
```

## Features

### Standard Features
- `serde` - Serde-Support für Konfiguration (standardmäßig aktiviert)

### Database Features
- `sqlite` - SQLite-Unterstützung
- `mysql` - MySQL/MariaDB-Unterstützung
- `all-drivers` - Alle Treiber aktivieren

```toml
# Nur SQLite
database_manager = { path = ".", features = ["sqlite"] }

# Nur MySQL
database_manager = { path = ".", features = ["mysql"] }

# Beide Treiber
database_manager = { path = ".", features = ["all-drivers"] }
```

## Beispiele

Weitere Beispiele findest du im `examples/` Verzeichnis:

```bash
# SQLite Beispiel
cargo run --example basic_usage --features sqlite

# MySQL Beispiel
cargo run --example basic_usage --features mysql

# Beide
cargo run --example basic_usage --features all-drivers
```

## Lizenz

MIT oder Apache-2.0, nach Wahl
