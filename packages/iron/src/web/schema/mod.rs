use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use crate::database::config::server_config;
use crate::database::DatabaseContext;

use crate::shared::config::ServerConfig;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn api_version(&self) -> &'static str {
        "0.1"
    }

    async fn server_config<'a>(
        &self,
        context: &Context<'a>,
        #[graphql(desc = "The name of the server")] name: String,
    ) -> async_graphql::Result<Option<ServerConfig>> {
        let connection_pool = &context.data::<DatabaseContext>()?.connection_pool;
        let mut connection = connection_pool.get()?;

        Ok(server_config(&name, &mut connection)?)
    }
}

pub type IronSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
