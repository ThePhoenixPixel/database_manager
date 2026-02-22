use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlite")]
use crate::drivers::sqlite::sqlite_config::DBSqliteConfig;


#[cfg(feature = "mysql")]
use crate::drivers::mysql::mysql_config::DBMysqlConfig;



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum DatabaseConfig {
    #[cfg(feature = "sqlite")]
    Sqlite(DBSqliteConfig),
    #[cfg(feature = "mysql")]
    Mysql(DBMysqlConfig),
}


