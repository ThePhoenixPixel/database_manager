use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::{Column};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::{Row as SqlxRow};

use crate::controller::DatabaseController;
use crate::drivers::sqlite::sqlite_config::DBSqliteConfig;
use crate::types::{
    TableSchema,
    Value,
    {FilterOperator, Row},
    {DbError, DbResult},
    {Column as DbColumn, ColumnType},
    OrderDirection,
    {DBSmallInt, DBInteger, DBBigInt, DBFloat, DBDouble, DBText, DBBoolean, DBBlob},
    QueryFilters,
    ForeignKeyAction,
    IndexType,
};

pub struct SqliteManager {
    config: DBSqliteConfig,
    pool: Option<SqlitePool>,
}

impl SqliteManager {
    pub fn new(config: DBSqliteConfig) -> Self {
        Self {
            config,
            pool: None,
        }
    }

    fn get_pool(&self) -> DbResult<&SqlitePool> {
        self.pool.as_ref().ok_or_else(|| {
            DbError::ConnectionError("Not connected to database".to_string())
        })
    }

    fn column_type_to_sql(&self, col_type: &ColumnType) -> String {
        match col_type {
            ColumnType::SmallInt => "INTEGER".to_string(),
            ColumnType::Integer => "INTEGER".to_string(),
            ColumnType::BigInt => "INTEGER".to_string(),
            ColumnType::Float => "REAL".to_string(),
            ColumnType::Double => "REAL".to_string(),
            ColumnType::Text => "TEXT".to_string(),
            ColumnType::VarChar(_) => "TEXT".to_string(), // SQLite doesn't enforce length
            ColumnType::Boolean => "INTEGER".to_string(), // SQLite uses 0/1 for boolean
            ColumnType::Date => "TEXT".to_string(),
            ColumnType::DateTime => "TEXT".to_string(),
            ColumnType::Timestamp => "TEXT".to_string(),
            ColumnType::Json => "TEXT".to_string(),
            ColumnType::Blob => "BLOB".to_string(),
        }
    }

    fn build_column_definition(&self, column: &DbColumn) -> String {
        let mut def = format!("{} {}", column.name, self.column_type_to_sql(&column.column_type));

        if column.primary_key {
            def.push_str(" PRIMARY KEY");
            if column.auto_increment {
                def.push_str(" AUTOINCREMENT");
            }
        }

        if !column.nullable && !column.primary_key {
            def.push_str(" NOT NULL");
        }

        if column.unique && !column.primary_key {
            def.push_str(" UNIQUE");
        }

        if let Some(default) = &column.default_value {
            def.push_str(&format!(" DEFAULT {}", self.value_to_sql(default)));
        }

        def
    }

    fn value_to_sql(&self, value: &Value) -> String {
        match value {
            Value::Null => "NULL".to_string(),
            Value::SmallInt(i) => i.0.to_string(),
            Value::Integer(i) => i.0.to_string(),
            Value::BigInt(i) => i.0.to_string(),
            Value::Float(f) => f.0.to_string(),
            Value::Double(f) => f.0.to_string(),
            Value::Text(s) => format!("'{}'", s.0.replace("'", "''")),
            Value::VarChar(s) => format!("'{}'", s.0.replace("'", "''")),
            Value::Boolean(b) => if b.0 { "1" } else { "0" }.to_string(),
            Value::Date(d) => format!("'{}'", d.0.replace("'", "''")),
            Value::DateTime(d) => format!("'{}'", d.0.replace("'", "''")),
            Value::Timestamp(d) => format!("'{}'", d.0.replace("'", "''")),
            Value::Json(j) => format!("'{}'", j.0.replace("'", "''")),
            Value::Blob(_) => "NULL".to_string(), // Binary data needs special handling
        }
    }

    fn row_from_sqlite(row: &SqliteRow) -> DbResult<Row> {
        let mut result = HashMap::new();

        for (idx, column) in row.columns().iter().enumerate() {
            let name = column.name().to_string();

            let value = if let Ok(v) = row.try_get::<i64, _>(idx) {
                Value::BigInt(DBBigInt(v))
            } else if let Ok(v) = row.try_get::<i32, _>(idx) {
                Value::Integer(DBInteger(v))
            } else if let Ok(v) = row.try_get::<i16, _>(idx) {
                Value::SmallInt(DBSmallInt(v))
            } else if let Ok(v) = row.try_get::<f64, _>(idx) {
                Value::Double(DBDouble(v))
            } else if let Ok(v) = row.try_get::<f32, _>(idx) {
                Value::Float(DBFloat(v))
            } else if let Ok(v) = row.try_get::<String, _>(idx) {
                Value::Text(DBText(v))
            } else if let Ok(v) = row.try_get::<bool, _>(idx) {
                Value::Boolean(DBBoolean(v))
            } else if let Ok(v) = row.try_get::<Vec<u8>, _>(idx) {
                Value::Blob(DBBlob(v))
            } else {
                Value::Null
            };

            result.insert(name, value);
        }

        Ok(result)
    }

    fn bind_value<'q>(query: sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>>, value: &'q Value) -> sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
        match value {
            Value::Null => query.bind(None::<String>),
            Value::SmallInt(i) => query.bind(i.0),
            Value::Integer(i) => query.bind(i.0),
            Value::BigInt(i) => query.bind(i.0),
            Value::Float(f) => query.bind(f.0),
            Value::Double(f) => query.bind(f.0),
            Value::Text(s) => query.bind(&s.0),
            Value::VarChar(s) => query.bind(&s.0),
            Value::Boolean(b) => query.bind(if b.0 { 1 } else { 0 }),
            Value::Date(d) => query.bind(&d.0),
            Value::DateTime(d) => query.bind(&d.0),
            Value::Timestamp(d) => query.bind(&d.0),
            Value::Json(j) => query.bind(&j.0),
            Value::Blob(b) => query.bind(&b.0),
        }
    }

    fn bind_value_as<'q, O>(query: sqlx::query::QueryAs<'q, sqlx::Sqlite, O, sqlx::sqlite::SqliteArguments<'q>>, value: &'q Value) -> sqlx::query::QueryAs<'q, sqlx::Sqlite, O, sqlx::sqlite::SqliteArguments<'q>> {
        match value {
            Value::Null => query.bind(None::<String>),
            Value::SmallInt(i) => query.bind(i.0),
            Value::Integer(i) => query.bind(i.0),
            Value::BigInt(i) => query.bind(i.0),
            Value::Float(f) => query.bind(f.0),
            Value::Double(f) => query.bind(f.0),
            Value::Text(s) => query.bind(&s.0),
            Value::VarChar(s) => query.bind(&s.0),
            Value::Boolean(b) => query.bind(if b.0 { 1 } else { 0 }),
            Value::Date(d) => query.bind(&d.0),
            Value::DateTime(d) => query.bind(&d.0),
            Value::Timestamp(d) => query.bind(&d.0),
            Value::Json(j) => query.bind(&j.0),
            Value::Blob(b) => query.bind(&b.0),
        }
    }
}

#[async_trait]
impl DatabaseController for SqliteManager {
    async fn connect(&mut self) -> DbResult<()> {
        let connection_string = format!("sqlite://{}", self.config.path());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;

        // Enable foreign keys for SQLite
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;

        self.pool = Some(pool);
        Ok(())
    }

    async fn disconnect(&mut self) -> DbResult<()> {
        if let Some(pool) = self.pool.take() {
            pool.close().await;
        }
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.pool.is_some()
    }

    async fn create_table(&self, schema: &TableSchema) -> DbResult<()> {
        let pool = self.get_pool()?;

        let column_defs: Vec<String> = schema.columns.iter()
            .map(|col| self.build_column_definition(col))
            .collect();

        let mut sql = format!(
            "CREATE TABLE {} (\n  {}\n",
            schema.name,
            column_defs.join(",\n  ")
        );

        // Add foreign keys
        for fk in &schema.foreign_keys {
            let action = |a: &ForeignKeyAction| match a {
                ForeignKeyAction::Cascade => "CASCADE",
                ForeignKeyAction::SetNull => "SET NULL",
                ForeignKeyAction::SetDefault => "SET DEFAULT",
                ForeignKeyAction::Restrict => "RESTRICT",
                ForeignKeyAction::NoAction => "NO ACTION",
            };

            sql.push_str(&format!(
                ",\n  FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE {} ON UPDATE {}",
                fk.columns.join(", "),
                fk.referenced_table,
                fk.referenced_columns.join(", "),
                action(&fk.on_delete),
                action(&fk.on_update)
            ));
        }

        sql.push_str("\n)");

        sqlx::query(&sql)
            .execute(pool)
            .await
            .map_err(|e| DbError::CreateTableError {
                table: schema.name.clone(),
                message: e.to_string(),
            })?;

        // Create indexes
        for index in &schema.indexes {
            let unique = match index.index_type {
                IndexType::Unique | IndexType::Primary => "UNIQUE ",
                _ => "",
            };

            let index_sql = format!(
                "CREATE {}INDEX {} ON {} ({})",
                unique,
                index.name,
                schema.name,
                index.columns.join(", ")
            );

            sqlx::query(&index_sql)
                .execute(pool)
                .await
                .map_err(|e| DbError::SchemaError {
                    table: Some(schema.name.clone()),
                    message: e.to_string(),
                })?;
        }

        Ok(())
    }

    async fn drop_table(&self, table_name: &str) -> DbResult<()> {
        let pool = self.get_pool()?;
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);

        sqlx::query(&sql)
            .execute(pool)
            .await
            .map_err(|e| DbError::DropTableError {
                table: table_name.to_string(),
                message: e.to_string(),
            })?;

        Ok(())
    }

    async fn is_table_exists(&self, table_name: &str) -> DbResult<bool> {
        let pool = self.get_pool()?;
        let sql = "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name = ?";

        let count: (i32,) = sqlx::query_as(sql)
            .bind(table_name)
            .fetch_one(pool)
            .await
            .map_err(|e| DbError::QueryError {
                table: None,
                message: e.to_string(),
            })?;

        Ok(count.0 > 0)
    }

    async fn list_tables(&self) -> DbResult<Vec<String>> {
        let pool = self.get_pool()?;
        let sql = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'";

        let rows = sqlx::query(sql)
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::QueryError {
                table: None,
                message: e.to_string(),
            })?;

        let tables = rows.iter()
            .map(|row| row.get::<String, _>(0))
            .collect();

        Ok(tables)
    }


    async fn get_table_schema(&self, table_name: &str) -> DbResult<TableSchema> {
        let pool = self.get_pool()?;

        let sql = format!("PRAGMA table_info({})", table_name);

        let rows = sqlx::query(&sql)
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::SchemaError {
                table: Some(table_name.to_string()),
                message: e.to_string(),
            })?;

        let mut schema = TableSchema::new(table_name);

        for row in rows {
            let name: String = row.get(1);
            let type_str: String = row.get(2);
            let not_null: i32 = row.get(3);
            let pk: i32 = row.get(5);

            let column_type = match type_str.to_uppercase().as_str() {
                "INTEGER" => ColumnType::Integer,
                "REAL" => ColumnType::Float,
                "TEXT" => ColumnType::Text,
                "BLOB" => ColumnType::Blob,
                _ => ColumnType::Text,
            };

            let mut column = DbColumn::new(name, column_type);

            if not_null == 0 {
                column = column.nullable();
            }

            if pk == 1 {
                column = column.primary_key();
            }

            schema = schema.add_column(column);
        }

        Ok(schema)
    }

    async fn insert(&self, table: &str, data: &Row) -> DbResult<Value> {
        let pool = self.get_pool()?;

        let mut columns: Vec<&String> = data.keys().collect();
        columns.sort(); // Sort to ensure consistent ordering

        let placeholders: Vec<String> = vec!["?"; columns.len()].iter().map(|s| s.to_string()).collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            columns.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "),
            placeholders.join(", ")
        );

        let mut query = sqlx::query(&sql);
        for column in &columns {
            let value = &data[*column];
            query = Self::bind_value(query, value);
        }

        let result = query
            .execute(pool)
            .await
            .map_err(|e| DbError::InsertError {
                table: table.to_string(),
                message: e.to_string(),
            })?;

        Ok(Value::BigInt(DBBigInt(result.last_insert_rowid())))
    }


    async fn query(&self, table: &str, filters: &QueryFilters) -> DbResult<Vec<Row>> {
        let pool = self.get_pool()?;

        let mut sql = format!("SELECT * FROM {}", table);

        if !filters.filters.is_empty() {
            let conditions: Vec<String> = filters.filters.iter()
                .map(|f| {
                    match &f.operator {
                        FilterOperator::Equals => format!("{} = ?", f.column),
                        FilterOperator::NotEquals => format!("{} != ?", f.column),
                        FilterOperator::GreaterThan => format!("{} > ?", f.column),
                        FilterOperator::LessThan => format!("{} < ?", f.column),
                        FilterOperator::GreaterOrEqual => format!("{} >= ?", f.column),
                        FilterOperator::LessOrEqual => format!("{} <= ?", f.column),
                        FilterOperator::Like => format!("{} LIKE ?", f.column),
                        FilterOperator::IsNull => format!("{} IS NULL", f.column),
                        FilterOperator::IsNotNull => format!("{} IS NOT NULL", f.column),
                        FilterOperator::In => {
                            let count = f.values.as_ref().map(|v| v.len()).unwrap_or(0);
                            let placeholders = vec!["?"; count].join(", ");
                            format!("{} IN ({})", f.column, placeholders)
                        }
                        FilterOperator::Between => format!("{} BETWEEN ? AND ?", f.column),
                    }
                })
                .collect();
            sql.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }

        if let Some(order) = &filters.order_by {
            let order_clauses: Vec<String> = order.iter()
                .map(|(col, dir)| {
                    let d = match dir {
                        OrderDirection::Asc => "ASC",
                        OrderDirection::Desc => "DESC",
                    };
                    format!("{} {}", col, d)
                })
                .collect();
            sql.push_str(&format!(" ORDER BY {}", order_clauses.join(", ")));
        }

        if let Some(limit) = filters.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = filters.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut query = sqlx::query(&sql);
        for filter in &filters.filters {
            match filter.operator {
                FilterOperator::IsNull | FilterOperator::IsNotNull => {
                    // No values to bind
                }
                FilterOperator::In | FilterOperator::Between => {
                    if let Some(values) = &filter.values {
                        for value in values {
                            query = Self::bind_value(query, value);
                        }
                    }
                }
                _ => {
                    // Check 'value' first (single value), then 'values' (array)
                    if let Some(value) = &filter.value {
                        query = Self::bind_value(query, value);
                    } else if let Some(values) = &filter.values {
                        if let Some(value) = values.first() {
                            query = Self::bind_value(query, value);
                        }
                    }
                }
            }
        }

        let rows = query
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::QueryError {
                table: Some(table.to_string()),
                message: e.to_string(),
            })?;

        rows.iter().map(Self::row_from_sqlite).collect()
    }

    async fn query_one(&self, table: &str, filters: &QueryFilters) -> DbResult<Option<Row>> {
        let mut filters_with_limit = filters.clone();
        filters_with_limit.limit = Some(1);

        let mut results = self.query(table, &filters_with_limit).await?;
        Ok(results.pop())
    }

    async fn update(&self, table: &str, filters: &QueryFilters, data: &Row) -> DbResult<usize> {
        let pool = self.get_pool()?;

        let mut keys: Vec<&String> = data.keys().collect();
        keys.sort(); // Sort to ensure consistent ordering

        let set_clauses: Vec<String> = keys.iter()
            .map(|key| format!("{} = ?", key))
            .collect();

        let mut sql = format!("UPDATE {} SET {}", table, set_clauses.join(", "));

        if !filters.filters.is_empty() {
            let conditions: Vec<String> = filters.filters.iter()
                .map(|f| format!("{} = ?", f.column))
                .collect();
            sql.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }

        let mut query = sqlx::query(&sql);

        // Bind SET values in same order as keys
        for key in &keys {
            query = Self::bind_value(query, &data[*key]);
        }

        // Bind WHERE values
        for filter in &filters.filters {
            if let Some(value) = &filter.value {
                query = Self::bind_value(query, value);
            } else if let Some(values) = &filter.values {
                if let Some(value) = values.first() {
                    query = Self::bind_value(query, value);
                }
            }
        }

        let result = query
            .execute(pool)
            .await
            .map_err(|e| DbError::UpdateError {
                table: table.to_string(),
                message: e.to_string(),
            })?;

        Ok(result.rows_affected() as usize)
    }

    async fn delete(&self, table: &str, filters: &QueryFilters) -> DbResult<usize> {
        let pool = self.get_pool()?;

        let mut sql = format!("DELETE FROM {}", table);

        if !filters.filters.is_empty() {
            let conditions: Vec<String> = filters.filters.iter()
                .map(|f| format!("{} = ?", f.column))
                .collect();
            sql.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }

        let mut query = sqlx::query(&sql);

        // Bind WHERE values
        for filter in &filters.filters {
            if let Some(value) = &filter.value {
                query = Self::bind_value(query, value);
            } else if let Some(values) = &filter.values {
                if let Some(value) = values.first() {
                    query = Self::bind_value(query, value);
                }
            }
        }

        let result = query
            .execute(pool)
            .await
            .map_err(|e| DbError::DeleteError {
                table: table.to_string(),
                message: e.to_string(),
            })?;

        Ok(result.rows_affected() as usize)
    }

    async fn count(&self, table: &str, filters: &QueryFilters) -> DbResult<usize> {
        let pool = self.get_pool()?;

        let mut sql = format!("SELECT COUNT(*) FROM {}", table);

        if !filters.filters.is_empty() {
            let conditions: Vec<String> = filters.filters.iter()
                .map(|f| format!("{} = ?", f.column))
                .collect();
            sql.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }

        let mut query = sqlx::query_as(&sql);

        // Bind WHERE values
        for filter in &filters.filters {
            if let Some(value) = &filter.value {
                query = Self::bind_value_as(query, value);
            } else if let Some(values) = &filter.values {
                if let Some(value) = values.first() {
                    query = Self::bind_value_as(query, value);
                }
            }
        }

        let count: (i32,) = query
            .fetch_one(pool)
            .await
            .map_err(|e| DbError::QueryError {
                table: Some(table.to_string()),
                message: e.to_string(),
            })?;

        Ok(count.0 as usize)
    }

}