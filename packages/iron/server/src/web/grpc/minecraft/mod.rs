use std::sync::Arc;

use sea_orm::Set;
use tonic::{Request, Response};
use tracing::{info, instrument};

use entity::minecraft_server::ActiveModel as MinecraftServerModel;
use iron_api::minecraft_service::minecraft_server_creator_server::MinecraftServerCreator;
use iron_api::minecraft_service::{MinecraftServerDefinition, PaperMcServerDefinition};
use iron_api::ServerCreateResponse;

use crate::database::insert::InsertModel;
use crate::database::IronDatabase;
use crate::manager::{execute_new, ApplicationManager};
use crate::web::grpc::{to_tonic_status, AppCreateContext};

mod aikars_flags;
pub mod papermc;

#[derive(Debug)]
pub struct IronMinecraftServerCreator {
    pub db: Arc<IronDatabase>,
    pub app_manager: Arc<ApplicationManager>,
}

#[tonic::async_trait]
impl MinecraftServerCreator for IronMinecraftServerCreator {
    #[instrument(skip(self))]
    async fn create_paper_mc_server(
        &self,
        request: Request<PaperMcServerDefinition>,
    ) -> tonic::Result<Response<ServerCreateResponse>> {
        let context = AppCreateContext::try_from(request.into_inner()).map_err(to_tonic_status)?;
        let id = context.application.properties.id;

        info!(?context.application.properties);

        self.db.insert(&context).await.map_err(to_tonic_status)?;
        execute_new(&self.app_manager, context.application)
            .await
            .map_err(to_tonic_status)?;

        Ok(Response::new(ServerCreateResponse { id: id.to_string() }))
    }
}

impl<M> InsertModel<MinecraftServerModel, AppCreateContext<M>> for MinecraftServerDefinition
where
    M: prost::Message,
{
    fn build_model(&self, context: &AppCreateContext<M>) -> anyhow::Result<MinecraftServerModel> {
        Ok(MinecraftServerModel {
            id: Set(context.application.properties.id.to_string()),
            game_version: Set(self.game_version.clone()),
        })
    }
}
