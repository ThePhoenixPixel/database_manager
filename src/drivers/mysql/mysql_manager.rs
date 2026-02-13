use std::collections::HashMap;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{Column, TypeInfo, ValueRef};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlRow};
use sqlx::{Row as SqlxRow};

use crate::drivers::mysql::mysql_config::DBMysqlConfig;
use crate::controller::DatabaseController;
use crate::types::{Column as DbColumn, *};

pub struct MysqlManager {
    config: DBMysqlConfig,
    pool: Option<MySqlPool>,
}

impl MysqlManager {
    pub fn new(config: DBMysqlConfig) -> Self {
        Self {
            config,
            pool: None,
        }
    }

    fn get_pool(&self) -> DbResult<&MySqlPool> {
        self.pool.as_ref().ok_or_else(|| {
            DbError::ConnectionError("Not connected to database".to_string())
        })
    }

    fn column_type_to_sql(&self, col_type: &ColumnType) -> String {
        match col_type {
            ColumnType::Int => "BIGINT".to_string(),
            ColumnType::UInt => "BIGINT UNSIGNED".to_string(),
            ColumnType::Float => "FLOAT".to_string(),
            ColumnType::Text => "TEXT".to_string(),
            ColumnType::VarChar(len) => format!("VARCHAR({})", len),
            ColumnType::Boolean => "BOOLEAN".to_string(),
            ColumnType::Date => "DATE".to_string(),
            ColumnType::DateTime => "DATETIME".to_string(),
            ColumnType::Timestamp => "TIMESTAMP".to_string(),
            ColumnType::Blob => "BLOB".to_string(),
        }
    }

    fn build_column_definition(&self, column: &DbColumn) -> String {
        let mut def = format!("{} {}", column.name, self.column_type_to_sql(&column.column_type));

        if !column.nullable {
            def.push_str(" NOT NULL");
        }

        if column.auto_increment {
            def.push_str(" AUTO_INCREMENT");
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
            Value::Int(i) => i.0.to_string(),
            Value::UInt(i) => i.0.to_string(),
            Value::Float(f) => f.0.to_string(),
            Value::Text(s) => format!("'{}'", s.0.replace("'", "\\'")),
            Value::VarChar(v) => format!("'{}'", v.value().replace("'", "\\'")),
            Value::Boolean(b) => if b.0 { "TRUE" } else { "FALSE" }.to_string(),
            Value::Date(d) => format!("'{}'", d.0.replace("'", "\\'")),
            Value::DateTime(d) => format!("'{}'", d.0.replace("'", "\\'")),
            Value::Timestamp(ts) => ts.0.to_string(),
            Value::Blob(_) => "NULL".to_string(), // Binary data needs special handling
        }
    }

    fn row_from_mysql(row: &MySqlRow) -> DbResult<Row> {
        let mut result = std::collections::HashMap::new();

        for (idx, column) in row.columns().iter().enumerate() {
            let name = column.name().to_string();

            // Prüfe auf die Typen, nullable Felder als Option abfragen
            let value = if let Ok(v) = row.try_get::<Option<u64>, _>(idx) {
                v.map(DBUInt).map(Value::UInt).unwrap_or(Value::Null)
            } else if let Ok(v) = row.try_get::<Option<i64>, _>(idx) {
                v.map(DBInt).map(Value::Int).unwrap_or(Value::Null)
            } else if let Ok(v) = row.try_get::<Option<f64>, _>(idx) {
                v.map(DBFloat).map(Value::Float).unwrap_or(Value::Null)
            } else if let Ok(v) = row.try_get::<Option<f32>, _>(idx) {
                v.map(|vv| DBFloat(vv as f64)).map(Value::Float).unwrap_or(Value::Null)
            } else if let Ok(v) = row.try_get::<Option<bool>, _>(idx) {
                v.map(DBBoolean).map(Value::Boolean).unwrap_or(Value::Null)
            } else if let Ok(v) = row.try_get::<Option<String>, _>(idx) {
                v.map(DBText).map(Value::Text).unwrap_or(Value::Null)
            } else if let Ok(v) = row.try_get::<Option<Vec<u8>>, _>(idx) {
                v.map(DBBlob).map(Value::Blob).unwrap_or(Value::Null)
            } else if let Ok(v) = row.try_get::<Option<chrono::NaiveDate>, _>(idx) {
                v.map(|vv| Value::Date(DBDate(vv.format("%Y-%m-%d").to_string())))
                    .unwrap_or(Value::Null)
            } else if let Ok(v) = row.try_get::<Option<chrono::NaiveDateTime>, _>(idx) {
                v.map(|vv| Value::DateTime(DBDatetime(vv.format("%Y-%m-%d %H:%M:%S").to_string())))
                    .unwrap_or(Value::Null)
            } else if let Ok(v) = row.try_get::<Option<chrono::NaiveDateTime>, _>(idx) {
                // Timestamp fallback
                v.map(|vv| Value::Timestamp(DBTimestamp(vv.timestamp())))
                    .unwrap_or(Value::Null)
            } else {
                // Alles andere oder nicht behandelbar → Null
                Value::Null
            };

            result.insert(name, value);
        }

        Ok(result)
    }



    fn bind_value<'q>(query: sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments>, value: &'q Value) -> sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments> {
        match value {
            Value::Null => query.bind(None::<String>),
            Value::Int(i) => query.bind(i.0),
            Value::UInt(i) => query.bind(i.0),
            Value::Float(f) => query.bind(f.0),
            Value::Text(s) => query.bind(&s.0),
            Value::VarChar(v) => query.bind(v.value()),
            Value::Boolean(b) => query.bind(b.0),
            Value::Date(d) => query.bind(&d.0),
            Value::DateTime(d) => query.bind(&d.0),
            Value::Timestamp(ts) => query.bind(ts.0),
            Value::Blob(b) => query.bind(&b.0),
        }
    }

    fn bind_value_as<'q, O>(query: sqlx::query::QueryAs<'q, sqlx::MySql, O, sqlx::mysql::MySqlArguments>, value: &'q Value) -> sqlx::query::QueryAs<'q, sqlx::MySql, O, sqlx::mysql::MySqlArguments> {
        match value {
            Value::Null => query.bind(None::<String>),
            Value::Int(i) => query.bind(i.0),
            Value::UInt(i) => query.bind(i.0),
            Value::Float(f) => query.bind(f.0),
            Value::Text(s) => query.bind(&s.0),
            Value::VarChar(v) => query.bind(v.value()),
            Value::Boolean(b) => query.bind(b.0),
            Value::Date(d) => query.bind(&d.0),
            Value::DateTime(d) => query.bind(&d.0),
            Value::Timestamp(ts) => query.bind(ts.0),
            Value::Blob(b) => query.bind(&b.0),
        }
    }
}

#[async_trait]
impl DatabaseController for MysqlManager {
    async fn connect(&mut self) -> DbResult<()> {
        let connection_string = self.config.get_connection_string();

        let pool = MySqlPoolOptions::new()
            .max_connections(self.config.get_pool_size())
            .connect(&connection_string)
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

        let mut column_defs: Vec<String> = schema.columns.iter()
            .map(|col| self.build_column_definition(col))
            .collect();

        // Add PRIMARY KEY constraint
        let pk_columns: Vec<&String> = schema.columns.iter()
            .filter(|col| col.primary_key)
            .map(|col| &col.name)
            .collect();

        if !pk_columns.is_empty() {
            column_defs.push(format!(
                "PRIMARY KEY ({})",
                pk_columns.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }

        // Add foreign keys
        for fk in &schema.foreign_keys {
            let action = |a: &ForeignKeyAction| match a {
                ForeignKeyAction::Cascade => "CASCADE",
                ForeignKeyAction::SetNull => "SET NULL",
                ForeignKeyAction::SetDefault => "SET DEFAULT",
                ForeignKeyAction::Restrict => "RESTRICT",
                ForeignKeyAction::NoAction => "NO ACTION",
            };

            column_defs.push(format!(
                "FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE {} ON UPDATE {}",
                fk.columns.join(", "),
                fk.referenced_table,
                fk.referenced_columns.join(", "),
                action(&fk.on_delete),
                action(&fk.on_update)
            ));
        }

        let sql = format!(
            "CREATE TABLE {} (\n  {}\n)",
            schema.name,
            column_defs.join(",\n  ")
        );

        sqlx::query(&sql)
            .execute(pool)
            .await
            .map_err(|e| DbError::CreateTableError {
                table: schema.name.clone(),
                message: e.to_string()
            })?;

        // Create indexes
        for index in &schema.indexes {
            let index_type = match index.index_type {
                IndexType::Unique => "UNIQUE INDEX",
                IndexType::FullText => "FULLTEXT INDEX",
                IndexType::Primary => continue, // Already handled
                _ => "INDEX",
            };

            let index_sql = format!(
                "CREATE {} {} ON {} ({})",
                index_type,
                index.name,
                schema.name,
                index.columns.join(", ")
            );

            sqlx::query(&index_sql)
                .execute(pool)
                .await
                .map_err(|e| DbError::SchemaError {
                    table: Some(schema.name.clone()),
                    message: e.to_string()
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
                message: e.to_string()
            })?;

        Ok(())
    }

    async fn is_table_exists(&self, table_name: &str) -> DbResult<bool> {
        let pool = self.get_pool()?;
        let sql = "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = DATABASE() AND table_name = ?";

        let count: (i64,) = sqlx::query_as(sql)
            .bind(table_name)
            .fetch_one(pool)
            .await
            .map_err(|e| DbError::QueryError {
                table: None,
                message: e.to_string()
            })?;

        Ok(count.0 > 0)
    }

    async fn list_tables(&self) -> DbResult<Vec<String>> {
        let pool = self.get_pool()?;
        let sql = "SELECT table_name FROM information_schema.tables WHERE table_schema = DATABASE()";

        let rows = sqlx::query(sql)
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::QueryError {
                table: None,
                message: e.to_string()
            })?;

        let tables = rows.iter()
            .map(|row| row.get::<String, _>(0))
            .collect();

        Ok(tables)
    }

    async fn get_table_schema(&self, table_name: &str) -> DbResult<TableSchema> {
        let pool = self.get_pool()?;

        let sql = "SELECT column_name, data_type, character_maximum_length, is_nullable, column_key, extra
                   FROM information_schema.columns
                   WHERE table_schema = DATABASE() AND table_name = ?
                   ORDER BY ordinal_position";

        let rows = sqlx::query(sql)
            .bind(table_name)
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::QueryError {
                table: None,
                message: e.to_string()
            })?;

        let mut schema = TableSchema::new(table_name);

        for row in rows {
            let name: String = row.get(0);
            let type_str: String = row.get(1);
            let char_max_length: Option<i64> = row.get(2);
            let is_nullable: String = row.get(3);
            let column_key: String = row.get(4);
            let extra: String = row.get(5);

            let column_type = match type_str.to_uppercase().as_str() {
                "TINYINT" | "SMALLINT" | "INT" | "INTEGER" | "BIGINT" | "MEDIUMINT" => ColumnType::Int,
                "FLOAT" | "DOUBLE" | "DECIMAL" | "NUMERIC" => ColumnType::Float,
                "VARCHAR" | "CHAR" => {
                    let len = char_max_length.unwrap_or(255) as usize;
                    ColumnType::VarChar(len)
                },
                "TEXT" | "LONGTEXT" | "MEDIUMTEXT" | "JSON" => ColumnType::Text,
                "BOOLEAN" | "TINYINT(1)" => ColumnType::Boolean,
                "DATE" => ColumnType::Date,
                "DATETIME" => ColumnType::DateTime,
                "TIMESTAMP" => ColumnType::Timestamp,
                "BLOB" | "LONGBLOB" | "MEDIUMBLOB" | "BINARY" | "VARBINARY" => ColumnType::Blob,
                _ => ColumnType::Text,
            };

            let mut column = DbColumn::new(name, column_type);

            if is_nullable == "YES" {
                column = column.nullable();
            }

            if column_key == "PRI" {
                column = column.primary_key();
            }

            if column_key == "UNI" {
                column = column.unique();
            }

            if extra.contains("auto_increment") {
                column = column.auto_increment();
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

        for col in &columns {
            let value = &data[*col];
            query = Self::bind_value(query, value);
        }

        let result = query
            .execute(pool)
            .await
            .map_err(|e| DbError::InsertError {
                table: table.to_string(),
                message: e.to_string()
            })?;

        Ok(Value::from(result.last_insert_id()))
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
                message: e.to_string()
            })?;

        rows.iter().map(Self::row_from_mysql).collect()
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
                message: e.to_string()
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
                message: e.to_string()
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

        let count: (i64,) = query
            .fetch_one(pool)
            .await
            .map_err(|e| DbError::QueryError {
                table: Some(table.to_string()),
                message: e.to_string()
            })?;

        Ok(count.0 as usize)
    }
}