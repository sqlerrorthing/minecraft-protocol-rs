use std::fmt::Debug;

use crate::codec::{decoder::VersionedDecoder, encoder::VersionedEncoder};

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
/// - [`VersionedEncoder`] to support serialization into bytes.
/// - [`VersionedDecoder`] to support deserialization from bytes.
pub trait Packet: Debug + Identifiable + VersionedEncoder + VersionedDecoder {}
