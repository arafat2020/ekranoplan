use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use transport::bus::{Bus, Processor, SessionId};
use transport::session::{DialerSession, ListenerSession, SessionError};
use transport::tcp::Identification;
use transport::tls::{TlsClient, TlsServer};

const OPERATING_SYSTEM: &str = "Linux";
const HOST_NAME: &str = "test-host";

fn fixture(name: &str) -> Vec<u8> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(name);
    std::fs::read(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

fn identification(tag: &str) -> Identification {
    Identification {
        version: "0.1.0".to_string(),
        operating_system: OPERATING_SYSTEM.to_string(),
        account_type: "User".to_string(),
        country: "Bangladesh".to_string(),
        country_code: "BD".to_string(),
        image_index: 0,
        id: "phase-06".to_string(),
        username: "arafat".to_string(),
        pc_name: HOST_NAME.to_string(),
        tag: tag.to_string(),
        encryption_key: String::new(),
        signature: Vec::new(),
        os: 3,
        capabilities: 0,
    }
}

/// Length-prefixed bytes that are not a valid `ClientIdentification` payload.
fn not_identification_frame() -> Vec<u8> {
    let payload = [0x0a, 0xff];
    let mut frame = (payload.len() as u32).to_le_bytes().to_vec();
    frame.extend_from_slice(&payload);
    frame
}

async fn tls_pair() -> (
    tokio_rustls::client::TlsStream<TcpStream>,
    tokio_rustls::server::TlsStream<TcpStream>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = TlsServer::from_pem(&fixture("server.crt"), &fixture("server.key")).unwrap();
    let accepted = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        drop(listener);
        server.accept(tcp).await.unwrap()
    });
    let tcp = TcpStream::connect(address).await.unwrap();
    let client = TlsClient::pin_pem(&fixture("server.crt")).unwrap();
    let client_tls = client.connect(tcp).await.unwrap();
    let server_tls = accepted.await.unwrap();
    (client_tls, server_tls)
}

struct Counting {
    session: SessionId,
    hits: Arc<Mutex<u32>>,
}

impl Processor for Counting {
    fn can_execute(&self, _message: &Identification) -> bool {
        true
    }

    fn can_execute_from(&self, session: SessionId) -> bool {
        session == self.session
    }

    fn execute(&self, _session: SessionId, _message: &Identification) {
        *self.hits.lock().expect("hits") += 1;
    }
}

#[tokio::test]
async fn handshake_then_following_frame_reaches_the_bus() {
    let (client, server) = tls_pair().await;
    let mut dialer = DialerSession::new(client);
    let mut listener = ListenerSession::new(server);
    let session = SessionId(1);
    let hits = Arc::new(Mutex::new(0));
    let bus = Bus::new();
    bus.register(Arc::new(Counting {
        session,
        hits: Arc::clone(&hits),
    }));

    let hello = identification("hello");
    let follow = identification("follow");

    let incoming = tokio::spawn(async move {
        let message = listener.read_identification().await.unwrap();
        (listener, message)
    });
    dialer.send_identification(&hello).await.unwrap();
    let (mut listener, message) = incoming.await.unwrap();
    assert_eq!(message.operating_system, OPERATING_SYSTEM);
    assert_eq!(message.pc_name, HOST_NAME);
    assert_eq!(message.tag, "hello");
    assert_eq!(*hits.lock().expect("hits"), 0);

    let result = tokio::spawn(async move {
        let accepted = dialer.receive_result().await.unwrap();
        (dialer, accepted)
    });
    listener.send_result(true).await.unwrap();
    let (mut dialer, accepted) = result.await.unwrap();
    assert!(accepted);
    assert!(dialer.is_identified());

    let delivered = tokio::spawn(async move {
        listener.receive_to_bus(&bus, session).await.unwrap();
    });
    dialer.send_following(&follow).await.unwrap();
    delivered.await.unwrap();
    assert_eq!(*hits.lock().expect("hits"), 1);
}

#[tokio::test]
async fn first_frame_that_is_not_identification_closes_the_socket() {
    let (mut client, server) = tls_pair().await;
    let mut listener = ListenerSession::new(server);
    let incoming = tokio::spawn(async move { listener.read_identification().await });

    client.write_all(&not_identification_frame()).await.unwrap();
    let err = incoming.await.unwrap().unwrap_err();
    assert!(matches!(err, SessionError::NotIdentification));

    let mut buf = [0u8; 8];
    let n = client.read(&mut buf).await.unwrap();
    assert_eq!(n, 0);
}

#[tokio::test]
async fn false_result_leaves_the_dialer_unidentified() {
    let (client, server) = tls_pair().await;
    let mut dialer = DialerSession::new(client);
    let mut listener = ListenerSession::new(server);

    let incoming = tokio::spawn(async move {
        listener.read_identification().await.unwrap();
        listener
    });
    dialer.send_identification(&identification("reject")).await.unwrap();
    assert!(
        !dialer.is_identified(),
        "the flag must stay false until the result arrives"
    );
    let mut listener = incoming.await.unwrap();

    let waiting = tokio::spawn(async move {
        let accepted = dialer.receive_result().await.unwrap();
        (dialer, accepted)
    });
    listener.send_result(false).await.unwrap();
    let (dialer, accepted) = waiting.await.unwrap();
    assert!(!accepted);
    assert!(!dialer.is_identified());
}
