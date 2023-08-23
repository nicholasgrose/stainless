use async_graphql::Context;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::database::config::server_config;
use crate::web::schema::game::server::GameServerConfig;

pub struct IronQueryRoot;

#[async_graphql::Object]
impl IronQueryRoot {
    async fn api_version(&self) -> &'static str {
        "0.1"
    }

    async fn server_config<'a>(
        &self,
        context: &Context<'a>,
        #[graphql(desc = "The id of the server")] id: Uuid,
    ) -> async_graphql::Result<Option<GameServerConfig>> {
        let connection = context.data::<DatabaseConnection>()?;

        Ok(server_config(id, connection).await?)
    }
}
