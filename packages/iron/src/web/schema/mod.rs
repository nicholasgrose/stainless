use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::{database::DatabaseContext, shared::config::ServerConfig};

pub struct Query;

#[graphql_object(context = DatabaseContext)]
impl Query {
    fn api_version() -> &'static str {
        "0.1"
    }

    async fn server_config(
        context: &DatabaseContext,
        #[graphql(description = "name of the server")] name: String,
    ) -> FieldResult<Option<ServerConfig>> {
        let DatabaseContext(context) = context;
        let database = context.read().await;

        Ok(database.server_config(&name)?)
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<DatabaseContext>, EmptySubscription<DatabaseContext>>;

pub fn new() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<DatabaseContext>::new(),
        EmptySubscription::<DatabaseContext>::new(),
    )
}
