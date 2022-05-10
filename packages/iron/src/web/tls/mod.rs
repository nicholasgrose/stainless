use std::{fs::File, io::BufReader};

use anyhow::{anyhow, Context};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub fn load_tls_config() -> crate::Result<ServerConfig> {
    let certificates = load_certificates("cert.pem").context("can't load certificate file")?;
    let private_key = load_private_key("key.pem").context("can't load private key file")?;

    Ok(ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certificates, private_key)
        .context("can't create tls configuration")?)
}

fn load_certificates(path: &str) -> crate::Result<Vec<Certificate>> {
    let mut cert_file = file_reader(path)?;

    Ok(certs(&mut cert_file)?
        .into_iter()
        .map(Certificate)
        .collect())
}

fn file_reader(path: &str) -> crate::Result<BufReader<File>> {
    let file = File::open(path)?;

    Ok(BufReader::new(file))
}

fn load_private_key(path: &str) -> crate::Result<PrivateKey> {
    let mut key_file = file_reader(path)?;
    let mut private_keys = pkcs8_private_keys(&mut key_file)?;

    if private_keys.get(0).is_none() {
        Err(anyhow!("no private key found").into())
    } else {
        Ok(PrivateKey(private_keys.swap_remove(0)))
    }
}
