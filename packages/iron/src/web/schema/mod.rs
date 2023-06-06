use crate::database::config::server_config;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use sea_orm::DatabaseConnection;

use crate::shared::config::GameServerConfig;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn api_version(&self) -> &'static str {
        "0.1"
    }

    async fn server_config<'a>(
        &self,
        context: &Context<'a>,
        #[graphql(desc = "The id of the server")] id: i32,
    ) -> async_graphql::Result<Option<GameServerConfig>> {
        let connection = context.data::<DatabaseConnection>()?;

        Ok(server_config(id, connection).await?)
    }
}

pub type IronSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
