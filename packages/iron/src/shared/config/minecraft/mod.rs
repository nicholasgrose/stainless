use self::papermc::PaperMC;

pub mod papermc;

#[derive(async_graphql::SimpleObject)]
pub struct Minecraft {
    pub jvm_runtime_arguments: Vec<String>,
    pub game_version: String,
    pub server: MinecraftServer,
}

#[derive(async_graphql::Union)]
pub enum MinecraftServer {
    PaperMC(PaperMC),
}
