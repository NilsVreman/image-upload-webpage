use std::io::{self, ErrorKind};

use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;

const CERTIFICATE_PATH: &str = ".vault/image-server-cert.pem";
const PRIVATE_KEY_PATH: &str = ".vault/image-server-key.pem";

pub fn load_tls_config() -> io::Result<ServerConfig> {
    let certs = load_certificates()?;
    let key = load_private_key()?;

    let mut tls_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    Ok(tls_config)
}

fn load_certificates() -> io::Result<Vec<CertificateDer<'static>>> {
    let certs: Vec<_> = CertificateDer::pem_file_iter(CERTIFICATE_PATH)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?
        .collect::<Vec<_>>();

    Ok(certs.into_iter().filter_map(|cert| cert.ok()).collect())
}

fn load_private_key() -> io::Result<PrivateKeyDer<'static>> {
    let key = PrivateKeyDer::from_pem_file(PRIVATE_KEY_PATH)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    Ok(key)
}
