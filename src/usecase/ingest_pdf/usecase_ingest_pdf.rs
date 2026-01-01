use pdfium_render::prelude::*;

use crate::domain::chunk::Chunk;

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

            let normalized = Self::normaize_text(&text);
            let blocks = Self::split_blocks(&normalized);
            let page_chunks = Self::chunk_blocks(blocks, 2000, 200);

            for c in page_chunks {
                chunks.push(Chunk {
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

    fn normaize_text(input: &str) -> String {
        let mut text = input.to_string();

        text = text.replace("-\n", "");

        text = text.replace("\r\n", "\n");

        text = text
            .lines()
            .map(|l| l.trim())
            .collect::<Vec<_>>()
            .join("\n");

        while text.contains("\n\n\n") {
            text = text.replace("\n\n\n", "\n\n")
        }

        text.trim().to_string()
    }

    fn split_blocks(text: &str) -> Vec<String> {
        text.split("\n\n")
            .map(|b| b.trim())
            .filter(|b| b.len() > 30) // buang noise kecil
            .map(|b| b.to_string())
            .collect()
    }

    fn chunk_blocks(blocks: Vec<String>, max_len: usize, overlap: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current = String::new();

        for block in blocks {
            if current.len() + block.len() > max_len {
                chunks.push(current.clone());

                let tail = current
                    .chars()
                    .rev()
                    .take(overlap)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>();

                current = tail + "\n" + &block;
            } else {
                if !current.is_empty() {
                    current.push('\n');
                }
                current.push_str(&block);
            }
        }

        if !current.is_empty() {
            chunks.push(current);
        }

        chunks
    }
}
