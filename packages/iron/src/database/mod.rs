pub mod config;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use tokio::sync::RwLock;
mod schema;

pub struct Database {
    connection_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    fn new(database_url: &str) -> crate::Result<Database> {
        Ok(Database {
            connection_pool: build_connection_pool(database_url)?,
        })
    }
}

pub struct DatabaseContext(pub RwLock<Database>);

impl DatabaseContext {
    pub fn new(database_url: &str) -> crate::Result<DatabaseContext> {
        Ok(DatabaseContext(RwLock::new(Database::new(database_url)?)))
    }
}

impl juniper::Context for DatabaseContext {}

fn build_connection_pool(
    database_url: &str,
) -> crate::Result<Pool<ConnectionManager<SqliteConnection>>> {
    let manager = ConnectionManager::new(database_url);

    Ok(Pool::builder().build(manager)?)
}
