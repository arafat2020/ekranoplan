use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use transport::tcp::{
    write_identification, FrameCodecError, FrameError, FrameReader, Identification, MAX_PAYLOAD_LEN,
};

fn identification() -> Identification {
    Identification {
        version: "0.1.0".to_string(),
        operating_system: "Linux".to_string(),
        account_type: "User".to_string(),
        country: "Bangladesh".to_string(),
        country_code: "BD".to_string(),
        image_index: 18,
        id: "phase-04".to_string(),
        username: "arafat".to_string(),
        pc_name: "loopback".to_string(),
        tag: "tcp".to_string(),
        encryption_key: "key".to_string(),
        signature: vec![1, 2, 3],
        os: 3,
        capabilities: 1,
    }
}

#[tokio::test]
async fn one_identification_crosses_loopback() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let message = identification();

    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        drop(listener);
        let mut reader = FrameReader::new(socket);
        reader.read_identification().await.unwrap()
    });

    let mut client = TcpStream::connect(address).await.unwrap();
    write_identification(&mut client, &message).await.unwrap();

    let received = server.await.unwrap();
    assert_eq!(received, message);
}

#[tokio::test]
async fn length_above_five_megabytes_stops_the_read() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        drop(listener);
        let mut reader = FrameReader::new(socket);
        let first = reader.read_identification().await;
        let second = reader.read_identification().await;
        (first, second)
    });

    let mut client = TcpStream::connect(address).await.unwrap();
    let len = MAX_PAYLOAD_LEN + 1;
    client.write_all(&len.to_le_bytes()).await.unwrap();

    let (first, second) = server.await.unwrap();
    assert!(matches!(
        first,
        Err(FrameError::Codec(FrameCodecError::TooLarge { len: got })) if got == len
    ));
    assert!(matches!(second, Err(FrameError::Stopped)));
}
