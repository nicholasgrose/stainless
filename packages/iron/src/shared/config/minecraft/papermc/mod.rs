#[derive(async_graphql::SimpleObject)]
pub struct PaperMC {
    pub project: String,
    pub build: i32,
}
