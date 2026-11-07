//! Ekranoplan server. Listens for agents and records identified sessions.

mod sessions;

use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};

use transport::bus::{Bus, SessionId};
use transport::session::{ListenerSession, SessionError};
use transport::tls::TlsServer;

use sessions::{SessionInfo, Sessions};

// Bind address, port, and certificate paths. Phase 13 replaces these.
const BIND_HOST: &str = "127.0.0.1";
const BIND_PORT: u16 = 4782;
const CERT_PATH: &str = "crates/transport/tests/fixtures/server.crt";
const KEY_PATH: &str = "crates/transport/tests/fixtures/server.key";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = std::fs::read(CERT_PATH)
        .map_err(|err| format!("read {CERT_PATH}: {err}"))?;
    let key = std::fs::read(KEY_PATH).map_err(|err| format!("read {KEY_PATH}: {err}"))?;
    let tls = TlsServer::from_pem(&cert, &key).map_err(|err| format!("tls: {err:?}"))?;

    let addr = format!("{BIND_HOST}:{BIND_PORT}");
    let listener = TcpListener::bind(&addr).await?;
    eprintln!("listening on {addr}");

    let sessions = Arc::new(Sessions::new());
    loop {
        let (tcp, peer) = listener.accept().await?;
        let tls = tls.clone();
        let sessions = Arc::clone(&sessions);
        tokio::spawn(async move {
            if let Err(err) = handle_connection(tcp, tls, sessions).await {
                eprintln!("connection {peer}: {err}");
            }
        });
    }
}

async fn handle_connection(
    tcp: TcpStream,
    tls: TlsServer,
    sessions: Arc<Sessions>,
) -> Result<(), String> {
    let stream = tls
        .accept(tcp)
        .await
        .map_err(|err| format!("tls handshake: {err:?}"))?;
    let mut session = ListenerSession::new(stream);

    let identification = match session.read_identification().await {
        Ok(message) => message,
        Err(SessionError::NotIdentification) => {
            return Err("first frame was not identification".to_string());
        }
        Err(err) => return Err(format!("identification: {err:?}")),
    };

    session
        .send_result(true)
        .await
        .map_err(|err| format!("send result: {err:?}"))?;

    let info = SessionInfo {
        operating_system: identification.operating_system.clone(),
        hostname: identification.pc_name.clone(),
        username: identification.username.clone(),
    };
    let id = sessions.insert(info.clone());
    println!(
        "session {id}: os={} hostname={} username={}",
        info.operating_system, info.hostname, info.username
    );

    wait_until_closed(&mut session, id).await;
    sessions.remove(id);
    Ok(())
}

/// Blocks until the peer closes or a read fails. Feature frames are not handled yet.
async fn wait_until_closed<S>(session: &mut ListenerSession<S>, id: u64)
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    let bus = Bus::new();
    let session_id = SessionId(id);
    loop {
        match session.receive_to_bus(&bus, session_id).await {
            Ok(()) => {}
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::{Path, PathBuf};
    use std::time::Duration;

    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;
    use tokio::time::timeout;

    use transport::session::DialerSession;
    use transport::tcp::Identification;
    use transport::tls::TlsClient;

    fn fixture(name: &str) -> Vec<u8> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../crates/transport/tests/fixtures");
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
            image_index: 0,
            id: "phase-10".to_string(),
            username: "arafat".to_string(),
            pc_name: "build".to_string(),
            tag: "phase-10".to_string(),
            encryption_key: String::new(),
            signature: Vec::new(),
            os: 3,
            capabilities: 0,
        }
    }

    fn not_identification_frame() -> Vec<u8> {
        let payload = [0x0a, 0xff];
        let mut frame = (payload.len() as u32).to_le_bytes().to_vec();
        frame.extend_from_slice(&payload);
        frame
    }

    async fn spawn_server() -> (std::net::SocketAddr, Arc<Sessions>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let tls = TlsServer::from_pem(&fixture("server.crt"), &fixture("server.key")).unwrap();
        let sessions = Arc::new(Sessions::new());
        let sessions_task = Arc::clone(&sessions);
        tokio::spawn(async move {
            loop {
                let Ok((tcp, _)) = listener.accept().await else {
                    break;
                };
                let tls = tls.clone();
                let sessions = Arc::clone(&sessions_task);
                tokio::spawn(async move {
                    let _ = handle_connection(tcp, tls, sessions).await;
                });
            }
        });
        (addr, sessions)
    }

    #[tokio::test]
    async fn identification_creates_a_session_and_close_removes_it() {
        let (addr, sessions) = spawn_server().await;

        let tcp = TcpStream::connect(addr).await.unwrap();
        let client = TlsClient::pin_pem(&fixture("server.crt")).unwrap();
        let tls = client.connect(tcp).await.unwrap();
        let mut dialer = DialerSession::new(tls);

        dialer
            .send_identification(&identification())
            .await
            .unwrap();
        assert!(dialer.receive_result().await.unwrap());

        timeout(Duration::from_secs(2), async {
            while sessions.len() != 1 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("session was recorded");

        let id = 0;
        assert!(sessions.contains(id));

        drop(dialer);

        timeout(Duration::from_secs(2), async {
            while sessions.contains(id) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("session was removed");
        assert_eq!(sessions.len(), 0);
    }

    #[tokio::test]
    async fn non_identification_first_frame_does_not_create_a_session() {
        let (addr, sessions) = spawn_server().await;

        let tcp = TcpStream::connect(addr).await.unwrap();
        let client = TlsClient::pin_pem(&fixture("server.crt")).unwrap();
        let mut tls = client.connect(tcp).await.unwrap();
        tls.write_all(&not_identification_frame()).await.unwrap();
        let _ = tls.shutdown().await;

        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_eq!(sessions.len(), 0);
    }

    #[test]
    fn certificate_fixtures_exist_for_manual_run() {
        assert!(Path::new(CERT_PATH).exists() || fixture_path("server.crt").exists());
        assert!(Path::new(KEY_PATH).exists() || fixture_path("server.key").exists());
    }

    fn fixture_path(name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../crates/transport/tests/fixtures");
        path.push(name);
        path
    }
}
