use std::env;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use anyhow::Context;
use http::Uri;
use sea_orm::Database;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tower::ServiceBuilder;
use tracing::warn;
use tracing::{info, instrument};

use iron_api::minecraft_service::minecraft_server_creator_server::MinecraftServerCreatorServer;

use crate::manager::ApplicationManager;
use crate::web::grpc::IronMinecraftServerCreator;

mod grpc;

#[derive(Debug)]
pub struct IronGrpcService {
    pub database_uri: Uri,
    pub tls: IronTlsConfig,
    pub address: SocketAddr,
}

#[derive(Debug)]
pub struct IronTlsConfig {
    pub certificate_file_path: PathBuf,
    pub key_file_path: PathBuf,
}

impl Default for IronGrpcService {
    fn default() -> Self {
        IronGrpcService {
            database_uri: env_var("IRON_DATABASE_URI", "sqlite://iron_db.sqlite3"),
            tls: IronTlsConfig {
                certificate_file_path: env_var("IRON_CERTIFICATE_PATH", "cert.pem"),
                key_file_path: env_var("IRON_KEY_PATH", "key.pem"),
            },
            address: SocketAddr::new(
                env_var("IRON_IP_ADDRESS", "127.0.0.1"),
                env_var("IRON_PORT", "8080"),
            ),
        }
    }
}

#[instrument]
fn env_var<U, T>(name: &str, default: U) -> T
where
    U: Into<String> + Display + Debug,
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    env::var(name)
        .with_context(|| "env var is missing")
        .and_then(|value| value.parse().with_context(|| "env var is set but invalid"))
        .or_else(|error| {
            warn!("falling back to default value due to {}", error);

            default
                .into()
                .parse()
                .with_context(|| format!("parsing failed for default value of env var {}", name))
        })
        .unwrap()
}

impl IronGrpcService {
    #[instrument]
    pub async fn start(&self) -> anyhow::Result<()> {
        info!("starting grpc service");

        let db_connection = Database::connect(self.database_uri.to_string()).await?;
        let app_manager = ApplicationManager::default();

        let tls_config = self.load_tls_config().await?;
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
                    app_manager,
                },
            ))
            .serve(self.address)
            .await
            .with_context(|| "Server experienced an error during execution")
    }

    async fn load_tls_config(&self) -> anyhow::Result<ServerTlsConfig> {
        let certificate = tokio::fs::read(&self.tls.certificate_file_path);
        let key = tokio::fs::read(&self.tls.key_file_path);
        let server_identity = Identity::from_pem(certificate.await?, key.await?);

        Ok(ServerTlsConfig::new().identity(server_identity))
    }
}
