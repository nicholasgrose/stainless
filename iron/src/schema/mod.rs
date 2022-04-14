use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};

use crate::database;
use crate::database::Database;

#[derive(Clone, GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn user(
        context: &Database,
        #[graphql(description = "id of the user")] id: i32,
    ) -> Option<&User> {
        context.get_user(&id)
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<database::Database>, EmptySubscription<Database>>;

pub fn new() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}
