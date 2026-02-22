use std::str::FromStr;
use address::Authority;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DBMysqlConfig {
    host: Authority,
    username: String,
    password: String,
    database: String,
    pool_size: u32,
}

impl DBMysqlConfig {
    pub fn new(host: Authority, username: String, password: String, database: String, pool_size: u32) -> DBMysqlConfig {
        DBMysqlConfig {
            host,
            username,
            password,
            database,
            pool_size,
        }
    }

    pub fn from_parts(host: &str, port: u16, username: &str, password: &str, database: &str, pool_size: u32) -> Result<DBMysqlConfig, String> {
        let host = Authority::from_str(format!("{}:{}", host, port).as_str()).map_err(|e| e.to_string())?;
        Ok(DBMysqlConfig {
            host,
            username: username.to_string(),
            password: password.to_string(),
            database: database.to_string(),
            pool_size,
        })
    }

    pub fn get_connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}/{}",
            self.username,
            self.password,
            self.host,
            self.database
        )
    }

    pub fn get_pool_size(&self) -> u32 {
        self.pool_size
    }
}







