use juniper::{EmptyMutation, EmptySubscription, FieldResult, graphql_object, RootNode};

use crate::{database::DatabaseContext, shared::config::ServerConfig};
use crate::database::config::server_config;

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
        let mut connection = context.connection_pool.get()?;

        let config_result = tokio::spawn(async {
            server_config(&name, &mut connection)
        }).await?;

        Ok(config_result?)
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
