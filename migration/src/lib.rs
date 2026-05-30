// migration/src/lib.rs
pub use sea_orm_migration::prelude::*;

mod m20260001_0000001_create_users;
mod m20260001_0000002_create_keys;
mod m20260001_0000003_create_files;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260001_0000001_create_users::Migration),
            Box::new(m20260001_0000002_create_keys::Migration),
            Box::new(m20260001_0000003_create_files::Migration),
        ]
    }
}
