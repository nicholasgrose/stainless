use std::time::Duration;

use anyhow::Context;
use sea_orm::Database;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tower::ServiceBuilder;

use iron_api::minecraft_service::minecraft_server_creator_server::MinecraftServerCreatorServer;

use crate::config::IronConfig;
use crate::manager::ApplicationManager;
use crate::web::grpc::IronMinecraftServerCreator;

mod grpc;

pub async fn start_server(config: IronConfig) -> anyhow::Result<()> {
    let db_connection = Database::connect(&config.database_uri).await?;
    let app_manager = ApplicationManager::default();

    let tls_config = load_tls_config(&config).await?;
    let middleware = ServiceBuilder::new()
        .load_shed()
        .timeout(Duration::from_secs(10));

    Server::builder()
        .tls_config(tls_config)?
        .trace_fn(|_| tracing::info_span!("iron_server"))
        .layer(middleware)
        .add_service(MinecraftServerCreatorServer::new(
            IronMinecraftServerCreator {
                db_connection,
                app_manager: app_manager.into(),
            },
        ))
        .serve(config.address)
        .await
        .with_context(|| "Server experienced an error during execution")
}

async fn load_tls_config(config: &IronConfig) -> anyhow::Result<ServerTlsConfig> {
    let certificate = tokio::fs::read(&config.tls.certificate_file_path);
    let key = tokio::fs::read(&config.tls.key_file_path);
    let server_identity = Identity::from_pem(certificate.await?, key.await?);

    Ok(ServerTlsConfig::new().identity(server_identity))
}
