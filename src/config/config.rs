use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlite")]
use crate::drivers::sqlite::DBSqliteConfig;


#[cfg(feature = "mysql")]
use crate::drivers::mysql::DBMysqlConfig;



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DatabaseConfig {
    #[cfg(feature = "sqlite")]
    Sqlite(DBSqliteConfig),
    #[cfg(feature = "mysql")]
    Mysql(DBMysqlConfig),
}


