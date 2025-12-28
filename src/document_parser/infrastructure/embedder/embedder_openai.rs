use axum::async_trait;

use crate::document_parser::ports::embedder::{EmbedError, Embedder, EmbeddingRequest, EmbeddingResponse};

pub struct OpenAiEmbedder{
    pub api_key: String,
}

#[async_trait]
impl Embedder for OpenAiEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbedError> {
        if text.trim().is_empty() {
            return Err(EmbedError::EmptyInput)
        }

        let client = reqwest::Client::new();

        let res = client
        .post("https://api.openai.com/v1/embeddings")
        .bearer_auth(&self.api_key)
        .json(&EmbeddingRequest{
            model: "text-embedding-3-large",
            input: text,
        })
        .send()
        .await
        .map_err(|e| EmbedError::Transport(e.to_string()))?;

        if !res.status().is_success() {
            return Err(EmbedError::Provider { 
                code: Some(res.status().as_u16()), 
                message: res.text().await.unwrap_or_default(),
            });
        }

        let body: EmbeddingResponse = res
        .json()
        .await
        .map_err(|e| EmbedError::InvalidResponse(e.to_string()))?;

        body.data
        .into_iter()
        .next()
        .map(|d| d.embedding)
        .ok_or_else(|| EmbedError::InvalidResponse("empty embedding".into()))
    }
}