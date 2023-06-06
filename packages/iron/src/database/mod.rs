use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};

pub mod config;

mod schema;


#[derive(Clone)]
pub struct DatabaseContext {
    pub connection_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DatabaseContext {
    pub fn new(database_url: &str) -> crate::Result<DatabaseContext> {
        Ok(DatabaseContext {
            connection_pool: build_connection_pool(database_url)?
        })
    }
}

fn build_connection_pool(
    database_url: &str,
) -> crate::Result<Pool<ConnectionManager<SqliteConnection>>> {
    let manager = ConnectionManager::new(database_url);

    Ok(Pool::builder().build(manager)?)
}
