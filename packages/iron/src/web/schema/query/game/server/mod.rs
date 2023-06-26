use self::minecraft::MinecraftConfig;

pub mod minecraft;

#[derive(async_graphql::SimpleObject)]
pub struct GameServerConfig {
    pub name: String,
    pub app: AppConfig,
}

#[derive(async_graphql::Union)]
pub enum AppConfig {
    Minecraft(MinecraftConfig),
}
