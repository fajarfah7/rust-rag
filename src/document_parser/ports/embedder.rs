use axum::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum EmbedError{
    // empty input text
    EmptyInput,

    // request to provider failed
    Transport(String),

    // provider response error
    Provider {
        code: Option<u16>,
        message: String,
    },

    // response not expected
    InvalidResponse(String),

    // quota limit
    RateLimited,

    // unknown
    Unknown(String)
}

#[derive(Debug, Serialize)]
pub struct EmbeddingRequest<'a> {
    pub model: &'a str,
    pub input: &'a str,
}
#[derive(Debug, Deserialize)]
pub struct EmbeddingResponse {
    pub data: Vec<EmbeddingData>,
}
#[derive(Debug, Deserialize)]
pub struct EmbeddingData {
    pub embedding: Vec<f32>
}

#[async_trait]
pub trait Embedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbedError>;
}