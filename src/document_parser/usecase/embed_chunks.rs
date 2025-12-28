use crate::document_parser::{domain::chunk::Chunk, ports::embedder::{EmbedError, Embedder}};

pub async fn embed_chunks<E: Embedder>(
    embedder: &E,
    chunks: &[Chunk],
) -> Result<Vec<Vec<f32>>, EmbedError> {
    let mut vectors = Vec::new();

    for c in chunks {
        let v = embedder.embed(&c.text).await?;
        vectors.push(v);
    }

    Ok(vectors)
}