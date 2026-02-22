use database_manager_derive::Table;
use database_manager::{
    DatabaseManager, Table,
    config::DatabaseConfig,
};
use std::collections::HashMap;
use database_manager::config::DBSqliteConfig;
use database_manager::types::*;

// Define a table using struct with derive macro
#[derive(Table)]
#[table_name = "users"]
struct Users {
    #[primary_key]
    #[auto_increment]
    id: DBUInt,

    name: DBText,
    email: DBText,
    age: DBInt,

    #[nullable]
    bio: DBText,
}

// Another table example
#[derive(Table)]
struct Posts {
    #[primary_key]
    #[auto_increment]
    id: DBUInt,

    user_id: DBInt,
    title: DBText,
    content: DBText,
    published: DBBoolean,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ORM-Style Derive Example ===\n");

    // Create database manager (ORM - database agnostic!)
    // You can easily switch between SQLite and MySQL by changing the config
    let sqlite_config = DBSqliteConfig::new("derive_example.db");
    let config = DatabaseConfig::Sqlite(sqlite_config);
    let mut manager = DatabaseManager::new(config)?;

    // Connect
    manager.connect().await?;
    println!("✓ Connected to database\n");

    // Sync tables (create if not exists)
    println!("Syncing tables...");
    Users::sync(&manager).await?;
    println!("✓ Users table synced");

    Posts::sync(&manager).await?;
    println!("✓ Posts table synced\n");

    // === INSERT DATA ===
    println!("=== Inserting Data ===\n");

    // Insert users
    let mut user1 = HashMap::new();
    user1.insert("name".to_string(), Value::from("Alice".to_string()));
    user1.insert("email".to_string(), Value::from("alice@example.com".to_string()));
    user1.insert("age".to_string(), Value::from(30));
    user1.insert("bio".to_string(), Value::from("Software developer".to_string()));

    let user1_id = manager.insert(Users::table_name(), &user1).await?;
    println!("✓ Inserted user Alice with ID: {:?}", user1_id);

    let mut user2 = HashMap::new();
    user2.insert("name".to_string(), Value::from("Bob".to_string()));
    user2.insert("email".to_string(), Value::from("bob@example.com".to_string()));
    user2.insert("age".to_string(), Value::from(25));
    user2.insert("bio".to_string(), Value::Null);

    let user2_id = manager.insert(Users::table_name(), &user2).await?;
    println!("✓ Inserted user Bob with ID: {:?}", user2_id);

    let mut user3 = HashMap::new();
    user3.insert("name".to_string(), Value::from("Charlie".to_string()));
    user3.insert("email".to_string(), Value::from("charlie@example.com".to_string()));
    user3.insert("age".to_string(), Value::from(35));
    user3.insert("bio".to_string(), Value::from("Tech lead".to_string()));

    let user3_id = manager.insert(Users::table_name(), &user3).await?;
    println!("✓ Inserted user Charlie with ID: {:?}\n", user3_id);

    // Insert posts
    let mut post1 = HashMap::new();
    post1.insert("user_id".to_string(), user1_id.clone());
    post1.insert("title".to_string(), Value::from("My First Post".to_string()));
    post1.insert("content".to_string(), Value::from("Hello World!".to_string()));
    post1.insert("published".to_string(), Value::from(true));

    let post1_id = manager.insert(Posts::table_name(), &post1).await?;
    println!("✓ Inserted post 1 with ID: {:?}", post1_id);

    let mut post2 = HashMap::new();
    post2.insert("user_id".to_string(), user1_id.clone());
    post2.insert("title".to_string(), Value::from("Draft Post".to_string()));
    post2.insert("content".to_string(), Value::from("Work in progress...".to_string()));
    post2.insert("published".to_string(), Value::from(false));

    let post2_id = manager.insert(Posts::table_name(), &post2).await?;
    println!("✓ Inserted post 2 with ID: {:?}\n", post2_id);

    // === QUERY ALL DATA ===
    println!("=== Getting All Users ===\n");
    let all_users = manager.query(Users::table_name(), &QueryFilters::new()).await?;
    for (i, user) in all_users.iter().enumerate() {
        println!("User {}:", i + 1);
        println!("  ID: {:?}", user.get("id"));
        println!("  Name: {:?}", user.get("name"));
        println!("  Email: {:?}", user.get("email"));
        println!("  Age: {:?}", user.get("age"));
        println!("  Bio: {:?}", user.get("bio"));
        println!();
    }

    // === FILTERED QUERIES ===
    println!("=== Getting Users with Filters ===\n");

    // Get user by email
    let mut filters = QueryFilters::new();
    //filters.add_filter(Filter::new("email", FilterOperator::Equals, Some(Value::Text("alice@example.com".to_string()))));

    let alice = manager.query(Users::table_name(), &filters).await?;
    println!("User with email 'alice@example.com':");
    for user in alice {
        println!("  Name: {:?}, Age: {:?}", user.get("name"), user.get("age"));
    }
    println!();

    // Get users older than 28
    let mut age_filter = QueryFilters::new();
   // age_filter.add(Filter::new("age", FilterOperator::GreaterThan, Some(Value::Integer(28))));

    let older_users = manager.query(Users::table_name(), &age_filter).await?;
    println!("Users older than 28:");
    for user in older_users {
        println!("  Name: {:?}, Age: {:?}", user.get("name"), user.get("age"));
    }
    println!();

    // Query one user
    let bob_result = manager.query_one(Users::table_name(), &{
        let mut f = QueryFilters::new();
    //    f.add(Filter::new("name", FilterOperator::Equals, Some(Value::Text("Bob".to_string()))));
        f
    }).await?;

    if let Some(bob) = bob_result {
        println!("Query one - Bob: {:?}\n", bob.get("email"));
    }

    // Get published posts
    println!("=== Getting Published Posts ===\n");
    let mut pub_filter = QueryFilters::new();
  //  pub_filter.add(Filter::new("published", FilterOperator::Equals, Some(Value::Boolean(true))));

    let published_posts = manager.query(Posts::table_name(), &pub_filter).await?;
    println!("Published posts: {}", published_posts.len());
    for post in published_posts {
        println!("  Title: {:?}, Content: {:?}", post.get("title"), post.get("content"));
    }
    println!();

    // === UPDATE DATA ===
    println!("=== Updating Data ===\n");

    let mut update_data = HashMap::new();
    update_data.insert("age".to_string(), Value::from(31));
    update_data.insert("bio".to_string(), Value::from("Senior Software Developer"));

    let mut update_filter = QueryFilters::new();
 //   update_filter.add(Filter::new("name", FilterOperator::Equals, Some(Value::Text("Alice".to_string()))));

    let updated_count = manager.update(Users::table_name(), &update_filter, &update_data).await?;
    println!("✓ Updated {} row(s)\n", updated_count);

    // Verify update
    let updated_alice = manager.query(Users::table_name(), &update_filter).await?;
    println!("Alice after update:");
    for user in updated_alice {
        println!("  Age: {:?}, Bio: {:?}", user.get("age"), user.get("bio"));
    }
    println!();

    // === COUNT ===
    println!("=== Counting Records ===\n");
    let total_users = manager.count(Users::table_name(), &QueryFilters::new()).await?;
    let total_posts = manager.count(Posts::table_name(), &QueryFilters::new()).await?;
    println!("Total users: {}", total_users);
    println!("Total posts: {}\n", total_posts);

    // === DELETE DATA ===
    println!("=== Deleting Data ===\n");

    let mut delete_filter = QueryFilters::new();
 //   delete_filter.add(Filter::new("name", FilterOperator::Equals, Some(Value::Text("Bob".to_string()))));

    let deleted_count = manager.delete(Users::table_name(), &delete_filter).await?;
    println!("✓ Deleted {} row(s)\n", deleted_count);

    // Verify deletion
    let remaining_users = manager.count(Users::table_name(), &QueryFilters::new()).await?;
    println!("Remaining users: {}\n", remaining_users);

    // Disconnect
    manager.disconnect().await?;
    println!("✓ Disconnected from database");

    Ok(())
}
