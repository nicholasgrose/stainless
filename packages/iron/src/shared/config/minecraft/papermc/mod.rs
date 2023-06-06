#[derive(async_graphql::SimpleObject)]
pub struct PaperMcConfig {
    pub project: String,
    pub build: i32,
}
