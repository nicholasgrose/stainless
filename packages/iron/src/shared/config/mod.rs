use self::minecraft::Minecraft;

pub mod minecraft;

#[derive(async_graphql::SimpleObject)]
pub struct ServerConfig {
    pub name: String,
    pub app: App,
}

#[derive(async_graphql::Union)]
pub enum App {
    Minecraft(Minecraft),
}
