use std::fmt::Debug;

use sea_orm::DatabaseConnection;

pub mod insert;

#[derive(Debug)]
pub struct IronDatabase {
    connection: DatabaseConnection,
}

impl From<DatabaseConnection> for IronDatabase {
    fn from(connection: DatabaseConnection) -> Self {
        IronDatabase { connection }
    }
}
