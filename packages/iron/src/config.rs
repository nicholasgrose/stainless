use std::env;
use std::net::SocketAddr;

use anyhow::Context;

pub struct IronConfig {
    pub database_uri: String,
    pub tls: IronTlsConfig,
    pub address: SocketAddr,
}

pub struct IronTlsConfig {
    pub certificate_file_path: String,
    pub key_file_path: String,
}

const IRON_DATABASE_URI: &str = "IRON_DATABASE_URI";
const IRON_CERTIFICATE_PATH: &str = "IRON_CERTIFICATE_PATH";
const IRON_KEY_PATH: &str = "IRON_KEY_PATH";
const IRON_IP_ADDRESS: &str = "IRON_IP_ADDRESS";
const IRON_PORT: &str = "IRON_PORT";

macro_rules! env_var_context {
    ($env_var:ident, $context:expr) => {
        || format!("environment variable {} is {}", $env_var, $context)
    };
}

fn env_var(name: &str) -> anyhow::Result<String> {
    env::var(name).with_context(env_var_context!(name, "missing"))
}

impl IronConfig {
    pub fn load() -> anyhow::Result<Self> {
        Ok(IronConfig {
            database_uri: env_var(IRON_DATABASE_URI)?,
            tls: IronTlsConfig {
                certificate_file_path: env_var(IRON_CERTIFICATE_PATH)?,
                key_file_path: env_var(IRON_KEY_PATH)?,
            },
            address: SocketAddr::new(
                env_var(IRON_IP_ADDRESS)?
                    .parse()
                    .with_context(env_var_context!(IRON_IP_ADDRESS, "invalid"))?,
                env_var(IRON_PORT)?
                    .parse()
                    .with_context(env_var_context!(IRON_PORT, "invalid"))?,
            ),
        })
    }
}

impl Default for IronConfig {
    fn default() -> Self {
        IronConfig {
            database_uri: "sqlite://iron_db.sqlite3".to_string(),
            tls: IronTlsConfig {
                certificate_file_path: "cert.pem".to_string(),
                key_file_path: "key.pem".to_string(),
            },
            address: SocketAddr::new("127.0.0.1".parse().unwrap(), 8080),
        }
    }
}
