use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EmbeddingRequest<'a> {
    pub model: &'a str,
    pub input: &'a str,
}