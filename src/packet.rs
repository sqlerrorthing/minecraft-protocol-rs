use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use byteorder::ByteOrder;

/// A trait representing an associated error type for encoding and decoding operations.
///
/// This trait defines a single associated type `Error` which must implement
/// the standard library's [`std::error::Error`] trait.
/// It is used as the error
/// type returned by encoding and decoding functions to represent failure cases.
pub trait ErrorType {
    type Error: std::error::Error;
}

/// Trait representing the ability to encode a type into a byte stream.
///
/// Types implementing `Encoder` can be serialized into a byte buffer,
/// respecting the specified byte order (endianness).
pub trait Encoder: ErrorType {
    /// Encodes the current value into the given buffer with specified byte order.
    ///
    /// # Parameters
    /// - `buffer`: The mutable writer to write encoded bytes into.
    /// - `order`: The byte order (endianness) to use during encoding.
    ///
    /// # Returns
    /// Returns `Ok(())` if encoding succeeds, or an error of type `Self::Error` otherwise.
    fn encode<W, O>(&self, buffer: &mut W) -> Result<(), Self::Error>
    where
        W: Write,
        O: ByteOrder;
}

/// Trait representing the ability to decode an instance of a type from a byte stream.
///
/// Types implementing `Decoder` can be constructed by reading bytes from a buffer,
/// respecting the specified byte order (endianness).
pub trait Decoder: ErrorType
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
    /// Returns `Ok(Self)` with the decoded instance if successful, or an error of type `Self::Error` otherwise.
    fn decode<R, O>(buffer: &mut R) -> Result<Self, Self::Error>
    where
        R: Read + Seek,
        O: ByteOrder;
}

/// Trait representing types which have a unique identifier.
///
/// This trait is designed for packet types in the Minecraft protocol,
/// where each packet has a constant `id` associated with it. The ID is used
/// for distinguishing packet types during encoding/decoding.
///
/// # Note
///
/// - The `id` function is an associated function (not a method on `self`) because
///   the ID is constant for a given packet type and does not depend on instance state.
pub trait Identifiable {
    fn id() -> u32;
}

/// Marker trait representing a Minecraft protocol packet.
///
/// This trait bounds several important behaviors that a Minecraft packet must
/// implement:
///
/// - [`Debug`] for debugging output.
/// - [`Identifiable`] to provide the packet's unique ID.
/// - [`Encoder`] to support serialization into bytes.
/// - [`Decoder`] to support deserialization from bytes.
pub trait Packet: Debug + Identifiable + Encoder + Decoder {}
