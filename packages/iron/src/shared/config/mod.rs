use diesel::Queryable;
use juniper::{GraphQLObject, GraphQLUnion};

use self::minecraft::Minecraft;

pub mod minecraft;

#[derive(GraphQLObject, Queryable)]
pub struct ServerConfig {
    pub name: String,
    #[diesel(embed)]
    pub app: App,
}

#[derive(GraphQLUnion)]
pub enum App {
    Minecraft(Minecraft),
}
