use crate::document_parser::{domain::chunk::Chunk, usecase::{ingest_pdf_helper::{chunk_blocks, normaize_text, split_blocks}}};
use pdfium_render::prelude::*;

pub struct IngestPdf;
impl IngestPdf {
    pub fn execute(document: &PdfDocument, source: &str) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut chunk_index = 0;

        for (idx, page) in document.pages().iter().enumerate() {
            let text = match page.text() {
                Ok(t) => t.all(),
                Err(e) => {
                    tracing::error!(error = ?e, "found error in a page");
                    continue;
                }
            };

            if text.is_empty() {
                continue;
            }

            let normalized = normaize_text(&text);
            let blocks = split_blocks(&normalized);
            let page_chunks = chunk_blocks(blocks, 2000, 200);

            for c in page_chunks {
                chunks.push(Chunk{
                    text: c,
                    page: idx + 1,
                    source: source.to_string(),
                    index: chunk_index,
                });
                chunk_index += 1;
            }

        }
        chunks
    }
}
