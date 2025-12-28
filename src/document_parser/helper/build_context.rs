use crate::document_parser::infrastructure::vector_storage::qdrant_upsert::QdrantPayload;

pub fn build_context(results: &[(QdrantPayload, f32)], max_chars: usize) -> String {
    let mut context = String::new();

    for (payload, score) in results {
        let snippet = format!(
            "[source: {}, page: {}, score: {:.3}]\n{}\n\n",
            payload.source, payload.page, score, payload.text,
        );

        if context.len() + snippet.len() > max_chars {
            break;
        }

        context.push_str(&snippet);
    }

    context
}

pub fn build_prompt(context: &str, question: &str) -> String {
    format!(
        r#"Kamu adalah asisten yang menjawab pertanyaan BERDASARKAN DOKUMEN.

        Jika jawaban tidak ditemukan di dokumen, katakan dengan jujur bahwa kamu tidak menemukannya.

        === DOKUMEN ===
        {context}

        === PERTANYAAN ===
        {question}

        === JAWABAN ==="#
    )
}
