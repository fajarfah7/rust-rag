use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::document_parser::domain::error::PdfParserError;

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
}

#[derive(Debug, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessageResponse,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessageResponse {
    pub content: String,
}

pub async fn ask_llm(base_url: &str, model: &str, prompt: &str) -> Result<String, PdfParserError> {
    let client = Client::new();

    let req = ChatRequest {
        model: model.to_string(),
        temperature: 0.2,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };

    let res = client
        .post(format!("{}/v1/chat/completions", base_url))
        .json(&req)
        .send()
        .await
        .map_err(|e| PdfParserError::UnknownError(format!("send request: {}", e.to_string())))?;

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
