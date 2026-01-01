use axum::async_trait;
use reqwest::Client;

use crate::{repository::llm::contract::Llm, request::llm::request_llm::{ChatMessage, ChatRequest}, response::llm::response_llm::ChatResponse};
use crate::error::pdf_parser::PdfParserError;

#[derive(Debug)]
pub struct LmStudio {
    client: Client,
    base_url: String,
    base_model: String,
}

impl LmStudio {
    pub fn new(client: Client, base_url: String, base_model: String) -> Self {
        Self {
            client,
            base_url,
            base_model,
        }
    }
}

#[async_trait]
impl Llm for LmStudio {
    async fn ask(
        &self,
        prompt: &str,
    ) -> Result<String, PdfParserError> {

        let req = ChatRequest {
            model: self.base_model.to_string(),
            temperature: 0.2,
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let res = self.client
            .post(format!("{}/v1/chat/completions", &self.base_url))
            .json(&req)
            .send()
            .await
            .map_err(|e| {
                PdfParserError::UnknownError(format!("send request: {}", e.to_string()))
            })?;

        let status = res.status();
        let text = res
            .text()
            .await
            .map_err(|e| PdfParserError::UnknownError(e.to_string()))?;

        // ptional debug
        // tracing::debug!("LLM STATUS: {}", status);
        // tracing::debug!("RAW LLM RESPONSE:\n{}", text);

        if !status.is_success() {
            return Err(PdfParserError::UnknownError(format!(
                "after sending: {}: {}",
                status, text
            )));
        }

        let body: ChatResponse = serde_json::from_str(&text).map_err(|e| {
            PdfParserError::UnknownError(format!("formating response: {}", e.to_string()))
        })?;

        body.choices
            .get(0)
            .map(|c| c.message.content.clone())
            .ok_or_else(|| PdfParserError::UnknownError("empty LLM response".into()))
    }
}
