use juniper::{GraphQLObject, GraphQLUnion};

use self::minecraft::Minecraft;

pub mod minecraft;

#[derive(Clone, GraphQLObject)]
pub struct ServerConfig {
    pub name: String,
    pub app: Game,
}

#[derive(Clone, GraphQLUnion)]
pub enum Game {
    Minecraft(Minecraft),
}
