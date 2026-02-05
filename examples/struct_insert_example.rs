use database_manager_derive::Table;
use database_manager::{
    DatabaseManager, Table,
    config::DatabaseConfig,
    drivers::sqlite::DBSqliteConfig,
    types::{DbText, DbInteger, Filter, QueryFilters, Value},
};

#[derive(Table, Debug)]
#[table_name = "users"]
struct User {
    #[primary_key]
    #[auto_increment]
    id: DbInteger,

    name: DbText,
    email: DbText,
    age: DbInteger,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Struct Insert & Query Example ===\n");

    // Setup
    let sqlite_config = DBSqliteConfig::new("test.db");
    let config = DatabaseConfig::Sqlite(sqlite_config);
    let mut manager = DatabaseManager::new(config)?;
    manager.connect().await?;

    // Sync table
    User::sync(&manager).await?;
    println!("✓ Table synced\n");

    // === INSERT via struct.create() ===
    println!("=== Inserting Users ===\n");

    let alice = User {
        id: DbInteger(0), // wird ignoriert wegen auto_increment
        name: DbText("Alice".to_string()),
        email: DbText("alice@example.com".to_string()),
        age: DbInteger(30),
    };

    let bob = User {
        id: DbInteger(0),
        name: DbText("Bob".to_string()),
        email: DbText("bob@example.com".to_string()),
        age: DbInteger(25),
    };

    let charlie = User {
        id: DbInteger(0),
        name: DbText("Charlie".to_string()),
        email: DbText("charlie@example.com".to_string()),
        age: DbInteger(35),
    };

    let alice_id = alice.create(&manager).await?;
    println!("✓ Alice inserted with ID: {:?}", alice_id);

    let bob_id = bob.create(&manager).await?;
    println!("✓ Bob inserted with ID: {:?}", bob_id);

    let charlie_id = charlie.create(&manager).await?;
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
    email_filter.add_filter(Filter::eq("email", Value::Text("alice@example.com".to_string())));

    let alice_users = User::find(&manager, &email_filter).await?;
    println!("User with email 'alice@example.com':");
    for user in alice_users {
        println!("  {:?}", user);
    }
    println!();

    // Find users older than 28
    let mut age_filter = QueryFilters::new();
    age_filter.add_filter(Filter::gt("age", Value::Integer(28)));

    let older_users = User::find(&manager, &age_filter).await?;
    println!("Users older than 28:");
    for user in older_users {
        println!("  Name: {:?}, Age: {:?}", user.name, user.age);
    }
    println!();

    // === FIND ONE via User::find_one() ===
    println!("=== Finding One User ===\n");

    let mut bob_filter = QueryFilters::new();
    bob_filter.add_filter(Filter::eq("name", Value::Text("Bob".to_string())));
    if let Some(bob_user) = User::find_one(&manager, &bob_filter).await? {
        println!("Found Bob: {:?}", bob_user);
    } else {
        println!("Bob not found!");
    }

    manager.disconnect().await?;
    println!("\n✓ Done!");

    Ok(())
}
