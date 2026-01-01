use axum::async_trait;

use crate::error::pdf_parser::PdfParserError;

#[async_trait]
pub trait Llm {
    async fn ask(&self, prompt: &str) -> Result<String, PdfParserError>;
}