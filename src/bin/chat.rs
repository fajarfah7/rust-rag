use rag::document_parser::{
    domain::error::PdfParserError,
    helper::build_context::{build_context, build_prompt},
    infrastructure::{
        embedder::embedder_lm_studio::LmStudioEmbedder, llm_chat::llm_chat::ask_llm, pdfium::loader::PdfLoader, vector_storage::{qdrant_search::search_qdrant, qdrant_upsert::upsert_to_qdrant}
    },
    ports::embedder::Embedder,
    usecase::{embed_chunks::embed_chunks, ingest_pdf::IngestPdf},
};

#[tokio::main]
async fn main() -> Result<(), PdfParserError> {
    // ========= CONFIG =========
    let pdf_path = "./sample.pdf";
    let collection = "pdf_chunks";
    let question = "Sebutkan Aset dan Keunggulan Kompetitif";

    let embedder = LmStudioEmbedder {
        base_url: "http://localhost:1234".into(),
        model: "nomic-ai/nomic-embed-text-v1.5-GGUF".into(),
    };

    let chat_model = "meta-llama-3-8b-instruct";
    let llm_base_url = "http://localhost:1234";

    // // // UNCOMMENT THESES TO EXTRACT DATA TO QDRANT
    // // ========= LOAD PDF =========
    // let pdf_loader = PdfLoader::new()?;
    // let document = pdf_loader.load(pdf_path)?;

    // // ========= INGEST =========
    // let chunks = IngestPdf::execute(&document, pdf_path);
    // println!("✅ chunks created: {}", chunks.len());

    // // ========= EMBEDDING =========
    // let vectors = embed_chunks(&embedder, &chunks).await?;
    // println!("✅ embeddings created: {}", vectors.len());

    // // ========= STORE =========
    // upsert_to_qdrant(collection, &chunks, &vectors).await?;
    // println!("✅ vectors upserted to qdrant");

    // ========= RETRIEVE =========
    let query_vector = embedder.embed(question).await?;
    let results = search_qdrant(collection, query_vector, 5).await?;
    println!("✅ retrieved {} relevant chunks", results.len());

    // ========= RAG ANSWER =========
    let context = build_context(&results, 5_000);
    let prompt = build_prompt(&context, question);

    let answer = ask_llm(
        llm_base_url,
        chat_model,
        &prompt,
    ).await?;

    println!("\n=== ANSWER ===\n{}\n", answer);

    Ok(())
}

