use std::env;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;
use http::Uri;
use sea_orm::{ConnectOptions, Database};
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tracing::log::LevelFilter;
use tracing::warn;
use tracing::{info, instrument};

use iron_api::minecraft_service::minecraft_server_creator_server::MinecraftServerCreatorServer;

use crate::database::IronDatabase;
use crate::manager::ApplicationManager;
use crate::web::grpc::minecraft::IronMinecraftServerCreator;

macro_rules! required_def {
    ($message:expr) => {
        $message.as_ref().context(format!(
            "required definition not provided: {}",
            stringify!($message)
        ))
    };
}

pub mod grpc;

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

impl IronGrpcService {
    pub fn new() -> Self {
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
        .context("env var is missing")
        .and_then(|value| value.parse().context("env var is set but invalid"))
        .or_else(|error| {
            warn!("falling back to default value due to {}", error);

            default.into().parse().context(format!(
                "parsing failed for default value of env var {}",
                name
            ))
        })
        .unwrap()
}

impl IronGrpcService {
    #[instrument]
    pub async fn start(&self) -> anyhow::Result<()> {
        info!("starting iron");

        let mut db_connect_options = ConnectOptions::new(self.database_uri.to_string());
        db_connect_options.sqlx_logging_level(LevelFilter::Debug);
        let db = IronDatabase::from(Database::connect(db_connect_options).await?).into();
        let app_manager = ApplicationManager::default().into();
        let tls_config = self.load_tls_config().await?;

        Server::builder()
            .tls_config(tls_config)?
            .trace_fn(|_| tracing::info_span!("iron_server"))
            .add_service(MinecraftServerCreatorServer::new(
                IronMinecraftServerCreator { db, app_manager },
            ))
            .serve(self.address)
            .await
            .context("server experienced an error during execution")
    }

    async fn load_tls_config(&self) -> anyhow::Result<ServerTlsConfig> {
        let certificate = tokio::fs::read(&self.tls.certificate_file_path).await?;
        let key = tokio::fs::read(&self.tls.key_file_path).await?;
        let server_identity = Identity::from_pem(certificate, key);

        Ok(ServerTlsConfig::new().identity(server_identity))
    }
}
