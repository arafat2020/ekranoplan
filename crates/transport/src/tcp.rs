//! Plain TCP frames. No TLS.

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use protocol::codec::{decode, encode};
use protocol::error::CodecError;
use protocol::identification::ClientIdentification;
use protocol::Message;

pub use protocol::codec::MAX_PAYLOAD_LEN;
pub use protocol::error::CodecError as FrameCodecError;
pub use protocol::identification::ClientIdentification as Identification;

/// A frame could not be written or read on a TCP socket.
#[derive(Debug)]
pub enum FrameError {
    Codec(CodecError),
    Io(std::io::Error),
    Message(String),
    /// A previous frame failed the codec check, so this socket is no longer read.
    Stopped,
}

/// Reads length-prefixed frames from one accepted socket or a stream wrapped around it.
pub struct FrameReader<S> {
    stream: S,
    stopped: bool,
}

impl<S> FrameReader<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            stopped: false,
        }
    }

    /// Reads one [`ClientIdentification`].
    ///
    /// A length above 5 MB returns [`FrameError::Codec`] and stops this reader.
    /// The next call returns [`FrameError::Stopped`] without reading again.
    pub async fn read_identification(&mut self) -> Result<ClientIdentification, FrameError> {
        if self.stopped {
            return Err(FrameError::Stopped);
        }

        let mut header = [0u8; 4];
        if let Err(err) = self.stream.read_exact(&mut header).await {
            self.stop().await;
            return Err(FrameError::Io(err));
        }

        let payload = match decode(&header) {
            Ok(payload) => payload.to_vec(),
            Err(CodecError::TooLarge { len }) => {
                self.stop().await;
                return Err(FrameError::Codec(CodecError::TooLarge { len }));
            }
            Err(CodecError::Incomplete { claimed, .. }) => {
                let mut body = vec![0u8; claimed];
                if let Err(err) = self.stream.read_exact(&mut body).await {
                    self.stop().await;
                    return Err(FrameError::Io(err));
                }
                let mut frame = Vec::with_capacity(4 + claimed);
                frame.extend_from_slice(&header);
                frame.extend_from_slice(&body);
                match decode(&frame) {
                    Ok(payload) => payload.to_vec(),
                    Err(err) => {
                        self.stop().await;
                        return Err(FrameError::Codec(err));
                    }
                }
            }
        };

        ClientIdentification::decode(payload.as_slice())
            .map_err(|err| FrameError::Message(err.to_string()))
    }

    async fn stop(&mut self) {
        self.stopped = true;
        let _ = self.stream.shutdown().await;
    }
}

/// Writes `message` as one length-prefixed frame.
pub async fn write_identification<S>(
    stream: &mut S,
    message: &ClientIdentification,
) -> Result<(), FrameError>
where
    S: AsyncWrite + Unpin,
{
    let frame = encode(&message.encode_to_vec()).map_err(FrameError::Codec)?;
    stream.write_all(&frame).await.map_err(FrameError::Io)?;
    Ok(())
}
