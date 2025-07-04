use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncRead;
use std::io::{Error, ErrorKind, Seek};
use std::io::{Read, Write};
use std::rc::Rc;
use std::sync::Arc;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use uuid::Uuid;
use crate::packet::{Encoder, Decoder};

macro_rules! impl_ordered_primitives {
    ($($ty:ty => $read_fn:ident, $write_fn:ident);+ $(;)?) => {
        $(
            #[async_trait::async_trait]
            impl Encoder for $ty {
                async fn encode<W>(&self, buffer: &mut W) -> Result<(), Error>
                where
                    W: AsyncWrite + Unpin + Send
                {
                    buffer.$write_fn(*self).await
                }
            }

            #[async_trait::async_trait]
            impl Decoder for $ty {
                async fn decode<R>(buffer: &mut R) -> Result<Self, Error>
                where
                    R: AsyncRead + Unpin + Send
                {
                    buffer.$read_fn().await
                }
            }
        )+
    };
}

impl_ordered_primitives! {
    i16 => read_i16, write_i16;
    i32 => read_i32, write_i32;
    i64 => read_i64, write_i64;

    u16 => read_u16, write_u16;
    u32 => read_u32, write_u32;
    u64 => read_u64, write_u64;

    f32 => read_f32, write_f32;
    f64 => read_f64, write_f64;
}

#[async_trait::async_trait]
impl Encoder for bool {
    async fn encode<W>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: AsyncWrite + Unpin + Send,
    {
        buffer.write_all(&[*self as u8]).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Decoder for bool {
    async fn decode<R>(buffer: &mut R) -> Result<Self, Error>
    where
        R: AsyncRead + Unpin + Send,
    {
        let mut byte = [0u8; 1];
        buffer.read_exact(&mut byte).await?;
        Ok(byte[0] != 0)
    }
}

macro_rules! impl_varint {
    ($name:ident, $int:ty, $uint:ty, $max_bytes:expr) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub $int);

        #[async_trait::async_trait]
        impl Encoder for $name {
            async fn encode<W>(&self, buffer: &mut W) -> Result<(), Error>
            where
                W: AsyncWrite + Unpin + Send,
            {
                let mut value = self.0 as $uint;
                while value & !0x7F != 0 {
                    buffer.write_all(&[((value & 0x7F) as u8) | 0x80]).await?;
                    value >>= 7;
                }
                buffer.write_all(&[value as u8]).await?;
                Ok(())
            }
        }

        #[async_trait::async_trait]
        impl Decoder for $name {
            async fn decode<R>(buffer: &mut R) -> Result<Self, Error>
            where
                R: AsyncRead + Unpin + Send,
            {
                let mut num_read = 0;
                let mut result: $uint = 0;

                loop {
                    let mut byte = [0u8];
                    buffer.read_exact(&mut byte).await?;
                    let b = byte[0];

                    result |= ((b & 0x7F) as $uint) << (7 * num_read);
                    num_read += 1;

                    if num_read > $max_bytes {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, concat!(stringify!($name), " too big")));
                    }

                    if b & 0x80 == 0 {
                        break;
                    }
                }

                Ok($name(result as $int))
            }
        }
    };
}

impl_varint!(VarI32, i32, u32, 5);
impl_varint!(VarI64, i64, u64, 10);

#[async_trait::async_trait]
impl<T> Encoder for Option<T>
where
    T: Encoder + Sync,
{
    async fn encode<W>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: AsyncWrite + Unpin + Send
    {
        match self {
            Some(value) => {
                true.encode(buffer).await?;
                value.encode(buffer).await
            }
            None => false.encode(buffer).await,
        }
    }
}

impl<T> Decoder for Option<T>
where
    T: Decoder,
{
    async fn decode<R>(buffer: &mut R) -> Result<Self, Error>
    where
        R: AsyncRead + Unpin + Send
    {
        let value = if bool::decode(buffer).await? {
            Some(T::decode(buffer).await?)
        } else {
            None
        };

        Ok(value)
    }
}

macro_rules! impl_smart_ptr_codecs {
    ($($ptr:ty),+ $(,)?) => {
        $(
            impl<T> Encoder for $ptr
            where
                T: Encoder,
            {
                fn encode<W, O>(&self, buffer: &mut W) -> Result<(), Error>
                where
                    W: Write,
                    O: ByteOrder,
                {
                    (**self).encode::<_,O>(buffer)
                }
            }

            impl<T> Decoder for $ptr
            where
                T: Decoder,
            {
                fn decode<R, O>(buffer: &mut R) -> Result<Self, Error>
                where
                    R: Read + Seek,
                    O: ByteOrder,
                {
                    let value = T::decode::<_, O>(buffer)?;
                    Ok(Self::new(value))
                }
            }
        )+
    };
}

impl_smart_ptr_codecs!(Arc<T>, Rc<T>, Box<T>);

impl<T> Encoder for Vec<T>
where
    T: Encoder,
{
    fn encode<W, O>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        O: ByteOrder,
    {
        VarI32(self.len() as _).encode::<_,O>(buffer)?;

        for item in self {
            item.encode::<_,O>(buffer)?;
        }

        Ok(())
    }
}

impl<T> Decoder for Vec<T>
where
    T: Decoder,
{
    fn decode<R, O>(buffer: &mut R) -> Result<Self, Error>
    where
        R: Read + Seek,
        O: ByteOrder,
    {
        let len = VarI32::decode::<_, O>(buffer)?;
        let len = len.0 as _;

        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(T::decode::<_, O>(buffer)?);
        }

        Ok(vec)
    }
}

impl Encoder for Vec<u8> {
    fn encode<W, O>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        O: ByteOrder,
    {
        VarI32(self.len() as _).encode::<_,O>(buffer)?;
        buffer.write_all(self)
    }
}

impl Decoder for Vec<u8> {
    fn decode<R, O>(buffer: &mut R) -> Result<Self, Error>
    where
        R: Read + Seek,
        O: ByteOrder,
    {
        let VarI32(len) = VarI32::decode::<_, O>(buffer)?;
        let len = len as usize;

        let mut vec = vec![0u8; len];
        buffer.read_exact(&mut vec)?;

        Ok(vec)
    }
}

impl<T> Encoder for &[T]
where
    T: Encoder,
{
    fn encode<W, O>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        O: ByteOrder,
    {
        VarI32(self.len() as i32).encode::<_,O>(buffer)?;
        for item in *self {
            item.encode::<_,O>(buffer)?;
        }
        Ok(())
    }
}

impl Encoder for &[u8] {
    fn encode<W, O>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        O: ByteOrder,
    {
        VarI32(self.len() as i32).encode::<_,O>(buffer)?;
        buffer.write_all(self).map_err(|e| e.into())
    }
}

impl Encoder for String {
    fn encode<W, O>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        O: ByteOrder
    {
        self.as_bytes().encode::<_,O>(buffer)
    }
}

impl Decoder for String {
    fn decode<R, O>(buffer: &mut R) -> Result<Self, Error>
    where
        R: Read + Seek,
        O: ByteOrder,
    {
        let bytes = Vec::<u8>::decode::<_, O>(buffer)?;
        String::from_utf8(bytes).map_err(|e| {
            Error::new(ErrorKind::InvalidData, e).into()
        })
    }
}

impl Encoder for Uuid {
    fn encode<W, O>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        O: ByteOrder,
    {
        buffer.write_all(self.as_bytes())
    }
}

impl Decoder for Uuid {
    fn decode<R, O>(buffer: &mut R) -> Result<Self, Error>
    where
        R: Read,
        O: ByteOrder,
    {
        let mut bytes = [0u8; 16];
        buffer.read_exact(&mut bytes)?;
        Ok(Uuid::from_bytes(bytes))
    }
}
