use futures::StreamExt;
use rag::{
    app_chat::{
        domain::chat_conversation::Conversation, repository::repository_chat::ChatRepository,
    },
    config::{environment::EnvConfig, kafka_consumer::create_kafka_consumer, postgre::new_pg_pool},
    helper::builder::build_prompt_summary,
    infrastructure::postgresql::chat_repository_sqlx::ChatRepositorySqlx,
    repository::{
        // embedder::embedder_lm_studio::LmStudioEmbedder,
        llm::{contract::Llm, lm_studio::LmStudio},
    },
    request::pagination::PaginationRequest,
};
use rdkafka::Message;
use reqwest::Client;

#[tokio::main]
async fn main() {
    rag::init_env();
    rag::init_tracing();
    tracing::info!("CONSUMER SUMMARY RUNNING");

    let cfg = EnvConfig::init();

    let group_id = "conversation-summary-consumer";
    let topics = &["conversation-summary"];
    let consumer = create_kafka_consumer(group_id, topics);

    let pool = new_pg_pool(&cfg.database).await;
    let repo_chat = ChatRepositorySqlx::new(pool);

    // let embedder = LmStudioEmbedder {
    //     base_url: "http://localhost:1234".into(),
    //     model: "nomic-ai/nomic-embed-text-v1.5-GGUF".into(),
    // };

    let client = Client::new();
    let llm = LmStudio::new(
        client,
        "http://localhost:1234".into(),
        "meta-llama-3-8b-instruct".into(),
    );

    // let offset = (page - 1) * per_page as u32;
    let message_pagination = PaginationRequest {
        page: Some(1),
        per_page: Some(15),
        offset: Some(0),
        search: None,
        sort: Some("-created_at".into()),
    };

    let mut stream = consumer.stream();
    while let Some(message) = stream.next().await {
        tracing::info!("RECEIVING MESSAGE");
        let m = match message {
            Ok(m) => m,
            Err(e) => {
                tracing::error!(error = ?e, "kafka error");
                continue;
            }
        };

        tracing::info!("VIEW MESSAGE");
        let payload = match m.payload_view::<str>() {
            Some(Ok(p)) => p,
            _ => continue,
        };

        tracing::info!("PARSE TO TYPE CONVERSATION");
        let conversation: Conversation = match serde_json::from_str(payload) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!(error = ?e, "invalid message payload");
                continue;
            }
        };

        tracing::info!("FINDING MESSAGES");
        let mut messages = match repo_chat
            .find_messages(&conversation.id, &message_pagination)
            .await
        {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!(error = ?e, "invalid message payload");
                continue;
            }
        };
        messages.reverse();

        // if messages.len() < 5 {
        //     continue;
        // }

        tracing::info!("FORMATTING MESSAGES");
        let conversation_messages = messages
            .iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n");

        if conversation_messages.trim().is_empty() {
            tracing::info!("found empty message, process will be skipped");
            continue;
        }

        tracing::info!("BUILD PROMPT");
        let prompt = build_prompt_summary(&conversation_messages);

        tracing::info!("REQUEST TO LLM");
        let summary = match llm.ask(&prompt).await {
            Ok(s) => s,
            Err(e) => {
                tracing::error!(error = ?e, "failed ask llm to summary conversation");
                continue;
            }
        };

        let summary: String = summary.trim().to_string();
        // Some(&summary) MUST ON RIGHT SIDE
        if conversation.summary.as_deref() == Some(&summary) {
            tracing::info!("skip process due to same summary information");
            continue;
        }

        match repo_chat
            .update_conversation_summary(&conversation.id, &summary)
            .await
        {
            Ok(_) => {
                tracing::info!("SUCCESS PROCESS SUMMARY")
            }
            Err(e) => {
                tracing::error!(error = ?e, "failed update summary");
                continue;
            }
        }
    }
}
