use rand::prelude::IteratorRandom;
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use uuid::Uuid;

use crate::entities::users;

pub fn generate_spool_key() -> String {
    format!("spool_{}", Uuid::new_v4().simple())
}

pub fn random_ass_string() -> String {
    let ammo = File::open("/home/tuxzilla/Projects/spool/src/words.txt") // stupid solution make work better later
        .expect("missing words.txt! this is how i generate passwords pls add it");
    let mag = BufReader::new(ammo);

    let lines = mag
        .lines()
        .map(|line| line.expect("failed to parse line row:"))
        .filter(|line| !line.trim().is_empty());

    let password = lines.choose_multiple(&mut rand::thread_rng(), 8);
    password.join("-")
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
        uid: Set(1),
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
