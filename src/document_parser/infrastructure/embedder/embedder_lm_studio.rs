use axum::async_trait;

use crate::document_parser::ports::embedder::{EmbedError, Embedder, EmbeddingRequest, EmbeddingResponse};

pub struct LmStudioEmbedder {
    pub base_url: String,
    pub model: String,
}

#[async_trait]
impl Embedder for LmStudioEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbedError> {
        if text.trim().is_empty() {
            return Err(EmbedError::EmptyInput);
        }

        let client = reqwest::Client::new();

        let res = client
            .post(format!("{}/v1/embeddings", self.base_url))
            .json(&EmbeddingRequest {
                model: &self.model,
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
