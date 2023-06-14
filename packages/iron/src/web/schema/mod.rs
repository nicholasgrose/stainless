use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::database::config::server_config;
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
        #[graphql(desc = "The id of the server")] id: String,
    ) -> async_graphql::Result<Option<GameServerConfig>> {
        let uuid = Uuid::parse_str(&id)?;
        let connection = context.data::<DatabaseConnection>()?;

        Ok(server_config(uuid, connection).await?)
    }
}

pub type IronSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
