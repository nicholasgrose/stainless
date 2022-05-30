use juniper::{GraphQLObject, GraphQLUnion};

use self::papermc::PaperMC;

pub mod papermc;

#[derive(Clone, GraphQLObject)]
pub struct Minecraft {
    pub jvm_runtime_arguments: Vec<String>,
    pub game_version: String,
    pub server: MinecraftServer,
}

#[derive(Clone, GraphQLUnion)]
pub enum MinecraftServer {
    PaperMC(PaperMC),
}
