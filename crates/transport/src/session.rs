//! Identification gate. Frames are not delivered to the bus until the handshake succeeds.

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use protocol::codec::{decode, encode};
use protocol::error::CodecError;
use protocol::identification::{ClientIdentification, ClientIdentificationResult};
use protocol::Message;

use crate::bus::{Bus, SessionId};

/// The handshake did not open the session.
#[derive(Debug)]
pub enum SessionError {
    Io(std::io::Error),
    Codec(CodecError),
    /// The first frame was not [`ClientIdentification`]. The socket is closed.
    NotIdentification,
    /// A frame arrived before identification succeeded, so it was not delivered.
    NotIdentified,
}

/// The listening side of one TLS session.
pub struct ListenerSession<S> {
    stream: S,
    identified: bool,
}

impl<S> ListenerSession<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            identified: false,
        }
    }

    pub fn is_identified(&self) -> bool {
        self.identified
    }

    /// Reads the handshake. A frame that is not [`ClientIdentification`] closes the socket.
    ///
    /// This does not deliver the frame to a bus.
    pub async fn read_identification(&mut self) -> Result<ClientIdentification, SessionError> {
        let payload = read_payload(&mut self.stream).await?;
        match ClientIdentification::decode(payload.as_slice()) {
            Ok(message) => Ok(message),
            Err(_) => {
                let _ = self.stream.shutdown().await;
                Err(SessionError::NotIdentification)
            }
        }
    }

    /// Replies to the handshake. The gate opens only when `result` is true.
    pub async fn send_result(&mut self, result: bool) -> Result<(), SessionError> {
        let message = ClientIdentificationResult { result };
        write_payload(&mut self.stream, &message.encode_to_vec()).await?;
        if result {
            self.identified = true;
        }
        Ok(())
    }

    /// Reads the next frame and runs it on `bus`.
    ///
    /// Before a true result has been sent, the frame is not delivered.
    pub async fn receive_to_bus(
        &mut self,
        bus: &Bus,
        session: SessionId,
    ) -> Result<(), SessionError> {
        if !self.identified {
            return Err(SessionError::NotIdentified);
        }
        let payload = read_payload(&mut self.stream).await?;
        let message = ClientIdentification::decode(payload.as_slice())
            .map_err(|_| SessionError::NotIdentification)?;
        bus.process(session, &message);
        Ok(())
    }
}

/// The dialing side of one TLS session.
pub struct DialerSession<S> {
    stream: S,
    identified: bool,
}

impl<S> DialerSession<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            identified: false,
        }
    }

    pub fn is_identified(&self) -> bool {
        self.identified
    }

    /// Sends the handshake. The session stays unidentified until [`receive_result`].
    pub async fn send_identification(
        &mut self,
        message: &ClientIdentification,
    ) -> Result<(), SessionError> {
        write_payload(&mut self.stream, &message.encode_to_vec()).await
    }

    /// Reads [`ClientIdentificationResult`]. A false result leaves the session unidentified.
    pub async fn receive_result(&mut self) -> Result<bool, SessionError> {
        let payload = read_payload(&mut self.stream).await?;
        let result = ClientIdentificationResult::decode(payload.as_slice())
            .map_err(|_| SessionError::NotIdentification)?;
        if result.result {
            self.identified = true;
        }
        Ok(result.result)
    }

    /// Sends a frame after identification. Used to show that a later frame can reach the bus.
    pub async fn send_following(
        &mut self,
        message: &ClientIdentification,
    ) -> Result<(), SessionError> {
        if !self.identified {
            return Err(SessionError::NotIdentified);
        }
        write_payload(&mut self.stream, &message.encode_to_vec()).await
    }
}

async fn read_payload<S>(stream: &mut S) -> Result<Vec<u8>, SessionError>
where
    S: AsyncRead + Unpin,
{
    let mut header = [0u8; 4];
    stream
        .read_exact(&mut header)
        .await
        .map_err(SessionError::Io)?;
    match decode(&header) {
        Ok(payload) => Ok(payload.to_vec()),
        Err(CodecError::TooLarge { len }) => Err(SessionError::Codec(CodecError::TooLarge { len })),
        Err(CodecError::Incomplete { claimed, .. }) => {
            let mut body = vec![0u8; claimed];
            stream
                .read_exact(&mut body)
                .await
                .map_err(SessionError::Io)?;
            let mut frame = Vec::with_capacity(4 + claimed);
            frame.extend_from_slice(&header);
            frame.extend_from_slice(&body);
            match decode(&frame) {
                Ok(payload) => Ok(payload.to_vec()),
                Err(err) => Err(SessionError::Codec(err)),
            }
        }
    }
}

async fn write_payload<S>(stream: &mut S, payload: &[u8]) -> Result<(), SessionError>
where
    S: AsyncWrite + Unpin,
{
    let frame = encode(payload).map_err(SessionError::Codec)?;
    stream.write_all(&frame).await.map_err(SessionError::Io)?;
    Ok(())
}
