use async_trait::async_trait;
use tokio::io::AsyncRead;

use crate::{codec::VersionedCodecError, protocol::ProtocolVersion};

/// Trait representing the ability to decode an instance of a type from a byte stream.
///
/// Types implementing `Decoder` can be constructed by reading bytes from a buffer,
/// respecting the specified byte order (endianness).
#[async_trait]
pub trait Decoder
where
    Self: Sized,
{
    /// Decodes an instance of the type from the given buffer with specified byte order.
    ///
    /// # Parameters
    /// - `buffer`: The mutable reader to read encoded bytes from.
    /// - `order`: The byte order (endianness) to use during decoding.
    ///
    /// # Returns
    /// Returns `Ok(Self)` with the decoded instance if successful,
    /// or an error of type `std::io:Error` otherwise.
    async fn decode<R>(buffer: &mut R) -> Result<Self, std::io::Error>
    where
        R: AsyncRead + Unpin + Send;
}

/// Trait for decoding an instance of a type from a byte stream with respect to a specific protocol version.
///
/// Types implementing `VersionedDecoder` can be constructed by reading bytes from a buffer,
/// applying version-aware decoding logic based on the provided `ProtocolVersion`.
#[async_trait]
pub trait VersionedDecoder
where
    Self: Sized,
{
    /// Asynchronously decodes an instance of the type from the provided byte stream,
    /// interpreting the data according to the specified protocol version.
    ///
    /// # Parameters
    /// - `buffer`: A mutable reference to a reader from which encoded bytes are read.
    /// - `source`: The `ProtocolVersion` indicating the version of the incoming data.
    ///
    /// # Returns
    /// Returns `Ok(Self)` containing the decoded instance on success,
    /// or a `VersionedCodecError` if decoding fails due to versioning or protocol errors.
    async fn decode<R>(
        buffer: &mut R,
        source: ProtocolVersion,
    ) -> Result<Self, VersionedCodecError>
    where
        R: AsyncRead + Unpin + Send;
}
