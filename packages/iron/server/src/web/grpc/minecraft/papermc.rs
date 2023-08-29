use anyhow::Context;
use uuid::Uuid;

use iron_api::minecraft_service::PaperMcServerDefinition;

use crate::manager::Application;
use crate::web::grpc::minecraft::aikars_flags::AikarsFlags;
use crate::web::grpc::AppCreateContext;

impl TryFrom<PaperMcServerDefinition> for AppCreateContext<PaperMcServerDefinition> {
    type Error = anyhow::Error;

    fn try_from(papermc_definition: PaperMcServerDefinition) -> Result<Self, Self::Error> {
        let minecraft_server_definition =
            required_def!(papermc_definition.minecraft_server_definition)?;
        let server_definition = required_def!(minecraft_server_definition.server_definition)?;

        Ok(AppCreateContext {
            application: Application {
                id: Uuid::new_v4(),
                name: server_definition.name.clone(),
                command: AikarsFlags::try_from(minecraft_server_definition)?.to_string(),
            },
            message: papermc_definition,
        })
    }
}
