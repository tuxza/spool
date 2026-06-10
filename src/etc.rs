use rand::prelude::SliceRandom;
use rand::thread_rng;

use sea_orm::Set;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use uuid::Uuid;

use crate::entities::users;

pub fn generate_spool_key() -> String {
    format!("spool_{}", Uuid::new_v4().simple())
}

pub fn random_ass_string() -> String {
    let words = include_str!("words.txt");
    let words: Vec<&str> = words.lines().collect();
    let mut random = thread_rng();
    let mut result = String::new();
    for _ in 0.. {
        result.push_str(words.choose(&mut random).unwrap());
        result.push(' ');
    }
    result
}

pub async fn db_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let db: DatabaseConnection = Database::connect("sqlite://cdn.db?mode=rwc").await?;
    Ok(db)
}

pub async fn setup_spool(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("no users found, creating admin user...");
    let api_key = generate_spool_key();
    let psd = random_ass_string();

    let admin_acc = users::ActiveModel {
        username: Set("admin".to_string()),
        psd: Set(psd.clone()),
        api_key: Set(api_key.clone()),
        ..Default::default()
    };
    admin_acc.insert(db).await?;
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    println!("WARNING! THIS WILL ONLY BE SHOWN ONCE!");
    println!("PLEASE COPY THESE KEYS AND STORE THEM SAFELY!");
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    println!("admin user created:");
    println!("username: admin");
    println!("password: {}", psd);
    println!("spool_key: {}", api_key);

    Ok(())
}
