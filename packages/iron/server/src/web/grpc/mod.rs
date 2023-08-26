use sea_orm::DatabaseConnection;
use tonic::{Request, Response};
use tracing::instrument;

use iron_api::minecraft_service::minecraft_server_creator_server::MinecraftServerCreator;
use iron_api::minecraft_service::PaperMcServerDefinition;
use iron_api::ServerCreateResponse;

#[derive(Debug, Default)]
pub struct IronMinecraftServerCreator {
    pub db: DatabaseConnection,
}

#[tonic::async_trait]
impl MinecraftServerCreator for IronMinecraftServerCreator {
    #[instrument]
    async fn create_paper_mc_server(
        &self,
        _: Request<PaperMcServerDefinition>,
    ) -> tonic::Result<Response<ServerCreateResponse>> {
        tracing::info!("creating papermc server");

        return Ok(Response::new(ServerCreateResponse {
            id: "1".to_string(),
        }));
    }
}
