use std::env;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use tracing::warn;

#[derive(Debug)]
pub struct IronConfig {
    pub database_uri: String,
    pub tls: IronTlsConfig,
    pub address: SocketAddr,
}

#[derive(Debug)]
pub struct IronTlsConfig {
    pub certificate_file_path: String,
    pub key_file_path: String,
}

const IRON_DATABASE_URI: &str = "IRON_DATABASE_URI";
const IRON_CERTIFICATE_PATH: &str = "IRON_CERTIFICATE_PATH";
const IRON_KEY_PATH: &str = "IRON_KEY_PATH";
const IRON_IP_ADDRESS: &str = "IRON_IP_ADDRESS";
const IRON_PORT: &str = "IRON_PORT";

fn env_var<T: FromStr>(name: &str, default: T) -> T {
    env::var(name)
        .with_context(|| "env variable {} is missing")
        .and_then(|value| {
            value.parse().map_err(|_| {
                let error = anyhow!("env variable {} is set but invalid", name);
                warn!("{}", error);

                error
            })
        })
        .unwrap_or(default)
}

impl Default for IronConfig {
    fn default() -> Self {
        IronConfig {
            database_uri: env_var(IRON_DATABASE_URI, "sqlite://iron_db.sqlite3".to_string()),
            tls: IronTlsConfig {
                certificate_file_path: env_var(IRON_CERTIFICATE_PATH, "cert.pem".to_string()),
                key_file_path: env_var(IRON_KEY_PATH, "key.pem".to_string()),
            },
            address: SocketAddr::new(
                env_var(IRON_IP_ADDRESS, "127.0.0.1".parse().unwrap()),
                env_var(IRON_PORT, 8080),
            ),
        }
    }
}
