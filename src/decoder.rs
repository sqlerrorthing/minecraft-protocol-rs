use async_trait::async_trait;
use tokio::io::AsyncRead;

/// Trait representing the ability to decode an instance of a type from a byte stream.
///
/// Types implementing `Decoder` can be constructed by reading bytes from a buffer,
/// respecting the specified byte order (endianness).
#[async_trait]
pub trait Decoder
where 
    Self: Sized
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
