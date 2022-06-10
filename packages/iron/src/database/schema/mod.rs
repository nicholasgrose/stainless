pub mod sql;

use diesel::Queryable;

#[derive(Queryable)]
pub struct ServerConfigRow {
    pub name: String,
}

#[derive(Queryable)]
pub struct ServerTypeRow {
    pub name: String,
    pub server_type: String,
}

#[derive(Queryable)]
pub struct MinecraftServerRow {
    pub name: String,
    pub game_version: String,
}

#[derive(Queryable)]
pub struct MinecraftTypeRow {
    pub name: String,
    pub minecraft_server_type: String,
}
