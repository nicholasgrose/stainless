use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;
use tonic::{Request, Response};
use tracing::instrument;

use iron_api::minecraft_service::minecraft_server_creator_server::MinecraftServerCreator;
use iron_api::minecraft_service::PaperMcServerDefinition;
use iron_api::ServerCreateResponse;

use crate::database::save_paper_mc_server;
use crate::manager::ApplicationManager;

#[derive(Debug)]
pub struct IronMinecraftServerCreator {
    pub db_connection: DatabaseConnection,
    pub app_manager: RwLock<ApplicationManager>,
}

#[tonic::async_trait]
impl MinecraftServerCreator for IronMinecraftServerCreator {
    #[instrument]
    async fn create_paper_mc_server(
        &self,
        request: Request<PaperMcServerDefinition>,
    ) -> tonic::Result<Response<ServerCreateResponse>> {
        let application = save_paper_mc_server(&self.db_connection, request.get_ref())
            .await
            .map_err(|err| tonic::Status::from_error(err.into()))?;
        let id = application.id;

        self.app_manager
            .write()
            .await
            .start_application(application)
            .await
            .map_err(|err| tonic::Status::from_error(err.into()))?;

        return Ok(Response::new(ServerCreateResponse { id: id.to_string() }));
    }
}
