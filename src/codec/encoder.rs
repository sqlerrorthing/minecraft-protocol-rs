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

/// Trait representing the ability to decode an instance of a type from a byte stream
/// while taking into account a specific protocol version.
///
/// Types implementing `VersionedDecoder` can be constructed by reading bytes from a buffer,
/// with decoding logic that may vary depending on the `ProtocolVersion` provided.
#[async_trait]
pub trait VersionedEncoder {
    /// Decodes an instance of the type from the given buffer,
    /// interpreting data according to the specified protocol version.
    ///
    /// # Parameters
    /// - `buffer`: The mutable reader to read encoded bytes from.
    /// - `source`: The protocol version from which the data originates.
    ///
    /// # Returns
    /// Returns `Ok(Self)` containing the decoded instance if successful,
    /// or a `VersionedCodecError` if decoding fails.
    async fn encode<W>(
        &self,
        buffer: &mut W,
        target: ProtocolVersion,
    ) -> Result<(), VersionedCodecError>
    where
        W: AsyncWrite + Unpin + Send;
}
