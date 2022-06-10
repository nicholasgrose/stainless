use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct PaperMC {
    pub project: String,
    pub build: i32,
}
