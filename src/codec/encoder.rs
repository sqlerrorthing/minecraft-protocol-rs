use async_trait::async_trait;
use tokio::io::AsyncWrite;

use crate::{codec::VersionedCodecError, protocol::ProtocolVersion};

/// Trait representing the ability to encode a type into a byte stream.
///
/// Types implementing `Encoder` can be serialized into a byte buffer,
/// respecting the specified byte order (endianness).
#[async_trait]
pub trait Encoder {
    /// Encodes the current value into the given buffer with specified byte order.
    ///
    /// # Parameters
    /// - `buffer`: The mutable writer to write encoded bytes into.
    /// - `order`: The byte order (endianness) to use during encoding.
    ///
    /// # Returns
    /// Returns `Ok(())` if encoding succeeds, or an error of type `std::io::Error` otherwise.
    async fn encode<W>(&self, buffer: &mut W) -> Result<(), std::io::Error>
    where
        W: AsyncWrite + Unpin + Send;
}

#[async_trait]
pub trait VersionedEncoder {
    async fn encode<W>(
        &self,
        buffer: &mut W,
        dest: ProtocolVersion,
    ) -> Result<(), VersionedCodecError>
    where
        W: AsyncWrite + Unpin + Send;
}
