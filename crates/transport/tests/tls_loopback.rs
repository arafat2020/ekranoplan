use std::path::PathBuf;

use tokio::net::{TcpListener, TcpStream};

use transport::tcp::{write_identification, FrameReader, Identification};
use transport::tls::{TlsClient, TlsServer};

fn fixture(name: &str) -> Vec<u8> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(name);
    std::fs::read(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

fn identification() -> Identification {
    Identification {
        version: "0.1.0".to_string(),
        operating_system: "Linux".to_string(),
        account_type: "User".to_string(),
        country: "Bangladesh".to_string(),
        country_code: "BD".to_string(),
        image_index: 18,
        id: "phase-05".to_string(),
        username: "arafat".to_string(),
        pc_name: "loopback".to_string(),
        tag: "tls".to_string(),
        encryption_key: "key".to_string(),
        signature: vec![4, 5, 6],
        os: 3,
        capabilities: 1,
    }
}

async fn exchange(pin_pem: &[u8]) -> Option<Identification> {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = TlsServer::from_pem(&fixture("server.crt"), &fixture("server.key")).unwrap();
    let message = identification();

    let accept = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        drop(listener);
        let tls = server.accept(tcp).await.ok()?;
        let mut reader = FrameReader::new(tls);
        reader.read_identification().await.ok()
    });

    let tcp = TcpStream::connect(address).await.unwrap();
    let client = TlsClient::pin_pem(pin_pem).unwrap();
    if let Ok(mut tls) = client.connect(tcp).await {
        write_identification(&mut tls, &message).await.unwrap();
    }

    accept.await.unwrap()
}

#[tokio::test]
async fn pin_match_delivers_one_identification_frame() {
    let received = exchange(&fixture("server.crt")).await;
    assert_eq!(received, Some(identification()));
}

#[tokio::test]
async fn pin_mismatch_delivers_zero_frames() {
    let received = exchange(&fixture("wrong.crt")).await;
    assert!(
        received.is_none(),
        "a certificate other than the pin must not deliver a frame"
    );
}
