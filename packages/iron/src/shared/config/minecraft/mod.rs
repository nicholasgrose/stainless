use self::papermc::PaperMcConfig;

pub mod papermc;

#[derive(async_graphql::SimpleObject)]
pub struct MinecraftConfig {
    pub jvm_runtime_arguments: Vec<String>,
    pub game_version: String,
    pub server: MinecraftServerConfig,
}

#[derive(async_graphql::Union)]
pub enum MinecraftServerConfig {
    PaperMC(PaperMcConfig),
}
