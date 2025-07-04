pub mod decoder;
pub mod encoder;

/// Errors that can occur during version-aware encoding or decoding of data.
///
/// This enum represents possible failures when handling protocol versions,
/// including I/O errors and unsupported protocol versions.
#[derive(Debug, thiserror::Error)]
pub enum VersionedCodecError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unsupported protocol version")]
    UnsupportedVersion,
}
