pub mod decoder;
pub mod encoder;

#[derive(Debug, thiserror::Error)]
pub enum VersionedCodecError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unsupported protocol version")]
    UnsupportedVersion,
}
