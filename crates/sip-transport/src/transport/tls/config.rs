use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use rustls::{Certificate, PrivateKey, ServerConfig, ClientConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use crate::error::{Error, Result};

pub fn load_certs(path: &str) -> Result<Vec<Certificate>> {
    let file = File::open(path).map_err(|e| Error::IoError(e))?;
    let mut reader = BufReader::new(file);
    let certs: Vec<Certificate> = certs(&mut reader)
        .map_err(|e| Error::IoError(e))?
        .into_iter()
        .map(Certificate)
        .collect();
    
    if certs.is_empty() {
        return Err(Error::TlsError(format!("No certificates found in {}", path)));
    }
    
    Ok(certs)
}

pub fn load_private_key(path: &str) -> Result<PrivateKey> {
    let file = File::open(path).map_err(|e| Error::IoError(e))?;
    let mut reader = BufReader::new(file);

    // Try PKCS8 first
    let keys = pkcs8_private_keys(&mut reader)
        .map_err(|e| Error::IoError(e))?
        .into_iter()
        .map(|der| PrivateKey(der))
        .collect::<Vec<_>>();
        
    if !keys.is_empty() {
        return Ok(keys[0].clone());
    }

    // Try RSA
    let file = File::open(path).map_err(|e| Error::IoError(e))?;
    let mut reader = BufReader::new(file);
    let keys = rsa_private_keys(&mut reader)
        .map_err(|e| Error::IoError(e))?
        .into_iter()
        .map(|der| PrivateKey(der))
        .collect::<Vec<_>>();

    if !keys.is_empty() {
        return Ok(keys[0].clone());
    }

    Err(Error::TlsError(format!("No private key found in {}", path)))
}

pub fn create_server_config(cert_path: &str, key_path: &str) -> Result<Arc<ServerConfig>> {
    let certs = load_certs(cert_path)?;
    let key = load_private_key(key_path)?;

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| Error::TlsError(e.to_string()))?;

    Ok(Arc::new(config))
}

pub fn create_client_config(ca_path: Option<&str>) -> Result<Arc<ClientConfig>> {
    let mut root_store = rustls::RootCertStore::empty();
    
    // Load system certs (optional, depending on environment)
    // For now, strict explicit CA loading
    
    if let Some(path) = ca_path {
        let certs = load_certs(path)?;
        for cert in certs {
            root_store.add(&cert).map_err(|e| Error::TlsError(e.to_string()))?;
        }
    } else {
        // In production, you might want webpki-roots here
    }

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Ok(Arc::new(config))
}
