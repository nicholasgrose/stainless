pub use sea_orm_migration::prelude::*;

mod m20230821_024525_create_server_tables;
mod m20230821_030342_create_minecraft_server_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230821_024525_create_server_tables::Migration),
            Box::new(m20230821_030342_create_minecraft_server_tables::Migration),
        ]
    }
}
