use juniper::{GraphQLObject, GraphQLUnion};

use self::minecraft::Minecraft;

pub mod minecraft;

#[derive(GraphQLObject)]
pub struct ServerConfig {
    pub name: String,
    pub app: App,
}

#[derive(GraphQLUnion)]
pub enum App {
    Minecraft(Minecraft),
}
