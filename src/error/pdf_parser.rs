use std::fmt::Display;

use crate::error::embedder::EmbedError;

#[derive(Debug)]
pub enum PdfParserError {
    UnknownError(String),
    FileNotFound(String),
    EmbedFailed(String),
}

impl Display for PdfParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfParserError::UnknownError(msg) => write!(f, "unknown error occured: {}", msg),
            PdfParserError::FileNotFound(msg) => write!(f, "file not found: {}", msg),
            PdfParserError::EmbedFailed(msg) => write!(f, "embed failed: {}", msg),
        }
    }
}

impl From<EmbedError> for PdfParserError {
    fn from(err: EmbedError) -> Self {
        match err {
            EmbedError::EmptyInput => PdfParserError::EmbedFailed("empty input".into()),

            EmbedError::RateLimited => PdfParserError::EmbedFailed("rate limited".into()),

            EmbedError::Transport(msg) => {
                PdfParserError::EmbedFailed(format!("transport error: {msg}"))
            }

            EmbedError::Provider { message, .. } => PdfParserError::EmbedFailed(message),

            EmbedError::InvalidResponse(msg) => PdfParserError::EmbedFailed(msg),

            EmbedError::Unknown(msg) => PdfParserError::UnknownError(msg),
        }
    }
}
