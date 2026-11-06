//! TLS on top of the plain frame streams. The dialer pins one server certificate.

use std::io::Cursor;
use std::sync::Arc;

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, DigitallySignedStruct, Error, ServerConfig, SignatureScheme};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream as ClientTlsStream;
use tokio_rustls::server::TlsStream as ServerTlsStream;
use tokio_rustls::{TlsAcceptor, TlsConnector};

/// A TLS handshake or PEM file could not be used.
#[derive(Debug)]
pub enum TlsError {
    Pem(String),
    Rustls(Error),
    Io(std::io::Error),
}

/// Accepts TCP sockets and wraps them with the server certificate.
#[derive(Clone)]
pub struct TlsServer {
    acceptor: TlsAcceptor,
}

impl TlsServer {
    /// Builds a server from a certificate PEM and a private key PEM.
    pub fn from_pem(cert_pem: &[u8], key_pem: &[u8]) -> Result<Self, TlsError> {
        install_provider();
        let certs = rustls_pemfile::certs(&mut Cursor::new(cert_pem))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| TlsError::Pem(err.to_string()))?;
        let key = rustls_pemfile::private_key(&mut Cursor::new(key_pem))
            .map_err(|err| TlsError::Pem(err.to_string()))?
            .ok_or_else(|| TlsError::Pem("private key is missing".to_string()))?;
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(TlsError::Rustls)?;
        Ok(Self {
            acceptor: TlsAcceptor::from(Arc::new(config)),
        })
    }

    /// Completes the server handshake. No application frame is read here.
    pub async fn accept(&self, stream: TcpStream) -> Result<ServerTlsStream<TcpStream>, TlsError> {
        self.acceptor.accept(stream).await.map_err(TlsError::Io)
    }
}

/// Dials a server and accepts only `pinned` as the end-entity certificate.
#[derive(Clone)]
pub struct TlsClient {
    connector: TlsConnector,
}

impl TlsClient {
    /// Builds a client that trusts exactly the certificate in `cert_pem`.
    pub fn pin_pem(cert_pem: &[u8]) -> Result<Self, TlsError> {
        install_provider();
        let pinned = first_cert(cert_pem)?;
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let verifier = Arc::new(PinnedServerCert { pinned, provider });
        let config = ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(verifier)
            .with_no_client_auth();
        Ok(Self {
            connector: TlsConnector::from(Arc::new(config)),
        })
    }

    /// Completes the client handshake. A certificate other than the pin fails here.
    pub async fn connect(
        &self,
        stream: TcpStream,
    ) -> Result<ClientTlsStream<TcpStream>, TlsError> {
        let name = ServerName::try_from("localhost")
            .map_err(|err| TlsError::Pem(err.to_string()))?
            .to_owned();
        self.connector
            .connect(name, stream)
            .await
            .map_err(TlsError::Io)
    }
}

fn install_provider() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}

fn first_cert(cert_pem: &[u8]) -> Result<CertificateDer<'static>, TlsError> {
    rustls_pemfile::certs(&mut Cursor::new(cert_pem))
        .next()
        .ok_or_else(|| TlsError::Pem("certificate is missing".to_string()))?
        .map_err(|err| TlsError::Pem(err.to_string()))
}

#[derive(Debug)]
struct PinnedServerCert {
    pinned: CertificateDer<'static>,
    provider: Arc<CryptoProvider>,
}

impl ServerCertVerifier for PinnedServerCert {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        if end_entity.as_ref() == self.pinned.as_ref() {
            Ok(ServerCertVerified::assertion())
        } else {
            Err(Error::General(
                "server certificate does not match the pin".into(),
            ))
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}
