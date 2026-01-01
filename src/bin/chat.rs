use rag::error::pdf_parser::PdfParserError;
use rag::{
    helper::builder::{build_context, build_prompt},
    repository::{
        embedder::{contract::Embedder, embedder_lm_studio::LmStudioEmbedder},
        llm::{contract::Llm, lm_studio::LmStudio},
    },
    usecase::qdrant::usecase_qdrant::search_qdrant,
};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), PdfParserError> {
    rag::init_tracing();

    // ========= CONFIG =========
    let collection = "pdf_chunks";
    let question = "berikan point point penting di BAB VII";

    let client = Client::new();
    let llm = LmStudio::new(
        client,
        "http://localhost:1234".into(),
        "meta-llama-3-8b-instruct".into(),
    );

    let embedder = LmStudioEmbedder {
        base_url: "http://localhost:1234".into(),
        model: "nomic-ai/nomic-embed-text-v1.5-GGUF".into(),
    };

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

    let answer = llm.ask(&prompt).await?;

    println!("\n=== ANSWER ===\n{}\n", answer);

    Ok(())
}
