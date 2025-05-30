use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecoderError {
    #[error("IO error while reading audio file: {0}")]
    Io(#[from] io::Error),

    #[error("Unsupported Audio format")]
    UnsupportedFormat,

    #[error("Failed to decode sample: {0}")]
    SampleError(String),
}

impl From<hound::Error> for DecoderError {
    fn from(value: hound::Error) -> Self {
        DecoderError::SampleError(value.to_string())
    }
}
