use diesel::Queryable;
use juniper::GraphQLObject;

#[derive(GraphQLObject, Queryable)]
pub struct PaperMC {
    pub project: String,
    pub build: i32,
}
