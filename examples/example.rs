use database_manager::config::{DatabaseConfig};
use database_manager::{DatabaseManager, Table, TableDerive};
use database_manager::types::*;



#[cfg(feature = "sqlite")]
use database_manager::config::DBSqliteConfig;

#[cfg(feature = "mysql")]
use database_manager::config::DBMysqlConfig;



#[derive(TableDerive, Debug, Clone)]
#[table_name("t_users")]
struct User {
    #[primary_key]
    #[auto_increment]
    id: DBUInt,

    name: DBText,
    email: DBText,

    #[nullable]
    age: Option<DBInt>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Struct Insert & Query Example ===\n");



    #[cfg(feature = "sqlite")]
    let sqlite_config = DatabaseConfig::Sqlite(DBSqliteConfig::new("test.db"));

    #[cfg(feature = "mysql")]
    let mysql_config = DatabaseConfig::Mysql(DBMysqlConfig::from_parts("192.168.178.22", 3306, "phoenix", "codergames2022", "phoenix", 10)?);


    // Create Database Manager
    #[cfg(feature = "sqlite")]
    let mut manager = DatabaseManager::new(sqlite_config)?;

    #[cfg(feature = "mysql")]
    let mut manager = DatabaseManager::new(mysql_config)?;



    manager.connect().await?;

    // Sync table
    User::sync(&manager).await?;
    println!("✓ Table synced\n");

    // === INSERT via struct.create() ===
    println!("=== Inserting Users ===\n");

    let alice = User {
        id: 0u16.into(), // ignore if auto_increment true
        name: "Alice".into(),
        email: "alice@example.com".into(),
        age: None,
    };

    let bob = User {
        id: 0u16.into(),
        name: "Bob".into(),
        email: "bob@example.com".into(),
        age: Some(25.into()),
    };

    let charlie = User {
        id: 0u16.into(),
        name: "Charlie".into(),
        email: "charlie@example.com".into(),
        age: Some(35.into()),
    };

    let alice_id = alice.insert(&manager).await?;
    println!("✓ Alice inserted with ID: {:?}", alice_id);

    let bob_id = bob.insert(&manager).await?;
    println!("✓ Bob inserted with ID: {:?}", bob_id);

    let charlie_id = charlie.insert(&manager).await?;
    println!("✓ Charlie inserted with ID: {:?}\n", charlie_id);

    // === GET ALL via User::all() ===
    println!("=== Getting All Users ===\n");
    let all_users = User::all(&manager).await?;
    println!("Found {} users:", all_users.len());
    for user in &all_users {
        println!("  - {:?}", user);
    }
    println!();

    // === FIND with filters via User::find() ===
    println!("=== Finding Users with Filters ===\n");

    // Find by email
    let mut email_filter = QueryFilters::new();
    email_filter.add_filter(Filter::eq("email", Value::from("alice@example.com")));

    let alice_users = User::find(&manager, &email_filter).await?;
    println!("User with email 'alice@example.com':");
    for user in alice_users {
        println!("  {:?}", user);
    }
    println!();

    // Find users older than 28
    let mut age_filter = QueryFilters::new();
    age_filter.add_filter(Filter::gt("age", Value::from(28)));

    let older_users = User::find(&manager, &age_filter).await?;
    println!("Users older than 28:");
    for user in older_users {
        println!("  Name: {:?}, Age: {:?}", user.name, user.age);
    }
    println!();

    // === FIND ONE via User::find_one() ===
    println!("=== Finding One User ===\n");

    let mut bob_filter = QueryFilters::new();
    bob_filter.add_filter(Filter::eq("name", Value::from("Bob")));
    if let Some(bob_user) = User::find_one(&manager, &bob_filter).await? {
        println!("Found Bob: {:?}", bob_user);
    } else {
        println!("Bob not found!");
    }

    manager.disconnect().await?;
    println!("\n✓ Done!");

    Ok(())
}
