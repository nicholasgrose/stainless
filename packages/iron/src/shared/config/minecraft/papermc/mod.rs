use juniper::GraphQLObject;

#[derive(Clone, GraphQLObject)]
pub struct PaperMC {
    pub project: String,
    pub build: i32,
}
