use sea_orm::Set;
use tonic::{Request, Response};
use tracing::{info, instrument};

use entity::minecraft_server::ActiveModel as MinecraftServerModel;
use iron_api::minecraft_service::minecraft_server_creator_server::MinecraftServerCreator;
use iron_api::minecraft_service::{MinecraftServerDefinition, PaperMcServerDefinition};
use iron_api::ServerCreateResponse;

use crate::database::insert::InsertModel;
use crate::database::IronDatabase;
use crate::manager::ApplicationManager;
use crate::web::grpc::{to_tonic_status, AppCreateContext};

mod aikars_flags;
pub mod papermc;

#[derive(Debug)]
pub struct IronMinecraftServerCreator {
    pub db: IronDatabase,
    pub app_manager: ApplicationManager,
}

#[tonic::async_trait]
impl MinecraftServerCreator for IronMinecraftServerCreator {
    #[instrument]
    async fn create_paper_mc_server(
        &self,
        request: Request<PaperMcServerDefinition>,
    ) -> tonic::Result<Response<ServerCreateResponse>> {
        let context = AppCreateContext::try_from(request.into_inner()).map_err(to_tonic_status)?;
        let id = context.application.id;

        info!("creating server {:?}", context);

        self.db.insert(&context).await.map_err(to_tonic_status)?;
        self.app_manager
            .execute_new(context.application)
            .await
            .map_err(to_tonic_status)?;

        return Ok(Response::new(ServerCreateResponse { id: id.to_string() }));
    }
}

impl<T> InsertModel<MinecraftServerModel, AppCreateContext<T>> for MinecraftServerDefinition
where
    T: prost::Message,
{
    fn build_model(&self, context: &AppCreateContext<T>) -> anyhow::Result<MinecraftServerModel> {
        Ok(MinecraftServerModel {
            id: Set(context.application.id.to_string()),
            game_version: Set(self.game_version.clone()),
        })
    }
}
