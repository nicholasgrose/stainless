use sea_orm::DatabaseConnection;
use tonic::{Request, Response};
use tracing::instrument;

use iron_api::minecraft_service::minecraft_server_creator_server::MinecraftServerCreator;
use iron_api::minecraft_service::PaperMcServerDefinition;
use iron_api::ServerCreateResponse;

use crate::database::save_paper_mc_server;

#[derive(Debug, Default)]
pub struct IronMinecraftServerCreator {
    pub db_connection: DatabaseConnection,
}

#[tonic::async_trait]
impl MinecraftServerCreator for IronMinecraftServerCreator {
    #[instrument]
    async fn create_paper_mc_server(
        &self,
        request: Request<PaperMcServerDefinition>,
    ) -> tonic::Result<Response<ServerCreateResponse>> {
        tracing::info!("creating papermc server");

        let id = save_paper_mc_server(&self.db_connection, request.get_ref())
            .await
            .map_err(|err| tonic::Status::from_error(err.into()))?;

        let response = Response::new(ServerCreateResponse { id });

        tracing::info!("{:?}", response);

        return Ok(response);
    }
}
