use crate::config::ServerConfig;
use crate::database;
use crate::database::Database;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> &'static str {
        "0.1"
    }

    fn server_config(
        context: &Database,
        #[graphql(description = "name of the server")] name: String,
    ) -> Option<&ServerConfig> {
        context.get_server(&name)
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
