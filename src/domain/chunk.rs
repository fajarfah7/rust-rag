#[derive(Debug, Clone)]
pub struct Chunk {
    pub text: String,
    pub page: usize,
    pub source: String,
    pub index: usize,
}
