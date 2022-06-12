pub mod sql;

use diesel::{Queryable, Selectable};
use sql::*;

#[derive(Queryable)]
pub struct ServerConfig {
    pub name: String,
}

#[derive(Queryable)]
pub struct ServerType {
    pub name: String,
    pub server_type: String,
}

#[derive(Queryable, Selectable)]
pub struct MinecraftServer {
    pub name: String,
    pub game_version: String,
}

#[derive(Queryable)]
pub struct MinecraftJvmArgument {
    pub name: String,
    pub argument: String,
}

#[derive(Queryable)]
pub struct MinecraftType {
    pub name: String,
    pub minecraft_server_type: String,
}

#[derive(Queryable, Selectable)]
pub struct PapermcServer {
    pub name: String,
    pub project: String,
    pub build: i32,
}
