use std::fmt::Debug;
use std::io::{Read, Write};
use byteorder::ByteOrder;

pub trait ErrorType {
    type Error: std::error::Error;
}

pub trait Encoder: ErrorType {
    fn encode<W, O>(&self, buffer: &mut W, order: O) -> Result<(), Self::Error>
    where
        W: Write,
        O: ByteOrder;
}

pub trait Decoder: ErrorType
where 
    Self: Sized
{
    fn decode<R, O>(buffer: &mut R, order: O) -> Result<Self, Self::Error>
    where
        R: Read,
        O: ByteOrder;
}

pub trait Packet: Debug + Encoder + Decoder {
    fn id() -> u32;
}