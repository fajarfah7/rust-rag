use std::{str::FromStr, sync::Arc};

use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, State},
    http::{HeaderValue},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{
    app_chat::{
        handler::{chat_adapter::string_to_stream, create_chat_request::CreateChatRequest},
        repository::repository_chat::ChatRepository, usecase::usecase_chat::ChatUsecase,
    },
    middleware::jwt_token::claims::Claims,
    repository::{embedder::contract::Embedder, llm::contract::Llm},
    request::{pagination::PaginationRequest, safe_query::SafeQuery},
    response::{error::ResponseError, success::ResponseSuccess},
};

pub async fn get_chats<R: ChatRepository, E: Embedder, L: Llm>(
    State(usecase): State<Arc<ChatUsecase<R, E, L>>>,
    Extension(claims): Extension<Claims>,
    SafeQuery(q): SafeQuery<PaginationRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    let user_id = Uuid::from_str(&claims.sub).map_err(|e| {
        tracing::error!(error = ?e, "failed parse claims");
        return ResponseError::InternalServerError;
    })?;

    let page = match q.page {
        Some(p) => p,
        None => 1,
    };

    let mut per_page: Option<u32> = None;
    let mut offset: Option<u32> = None;
    match q.per_page {
        Some(pp) => {
            per_page = Some(pp);
            offset = Some((page - 1) * pp as u32);
        }
        None => (),
    };

    let sort = match q.sort {
        Some(s) => s,
        None => "-created_at".into(),
    };
    let query = PaginationRequest {
        page: Some(page),
        per_page: per_page,
        offset: offset,
        search: Some("".into()),
        sort: Some(sort),
    };

    let result = usecase.get_chats(&user_id, &query).await?;

    Ok(ResponseSuccess::Pagination(
        page,
        per_page,
        result.total_data as u64,
        Some(result.data),
    ))
}

pub async fn get_chat_messages<R: ChatRepository, E: Embedder, L: Llm>(
    State(usecase): State<Arc<ChatUsecase<R, E, L>>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    SafeQuery(q): SafeQuery<PaginationRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    let user_id = Uuid::from_str(&claims.sub).map_err(|e| {
        tracing::error!(error = ?e, "failed parse claims");
        return ResponseError::InternalServerError;
    })?;

    let conversation_id = Uuid::from_str(&id).map_err(|e| {
        tracing::error!(error = ?e, "failed parse claims");
        return ResponseError::InternalServerError;
    })?;

    let page = match q.page {
        Some(p) => p,
        None => 1,
    };
    let per_page = match q.per_page {
        Some(pp) => pp,
        None => 15,
    };
    let offset = (page - 1) * per_page as u32;
    let sort = match q.sort {
        Some(s) => s,
        None => "-created_at".into(),
    };

    let query = PaginationRequest {
        page: Some(page),
        per_page: Some(per_page),
        offset: Some(offset),
        search: Some("".into()),
        sort: Some(sort),
    };

    let result = usecase
        .get_chat_messages(&user_id, &conversation_id, &query)
        .await?;

    Ok(ResponseSuccess::Pagination(
        page,
        Some(per_page),
        result.total_data as u64,
        Some(result.data),
    ))
}

pub async fn chat<R: ChatRepository, E: Embedder, L: Llm>(
    State(usecase): State<Arc<ChatUsecase<R, E, L>>>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateChatRequest>,
) -> Result<Response<Body>, ResponseError> {
    let user_id = Uuid::from_str(&claims.sub).map_err(|_| return ResponseError::Unauthorized)?;

    let response = usecase
        .create_chat(&user_id, req.conversation_id, &req.message)
        .await?;

    let answer = response.answer;
    let conversation_id = response.conversation_id;

    // // THIS IS USED FOR TESTING PURPOST
    // let answer = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string();
    // let conversation_id = "79e6826d-b139-4b13-b652-aa1cb0cf87a0".to_string();

    let stream = string_to_stream(answer);

    let body = Body::from_stream(stream);

    let mut res = Response::new(body);
    res.headers_mut().insert(
        "X-Conversation-Id",
        HeaderValue::from_str(&conversation_id.to_string()).unwrap(),
    );
    res.headers_mut().insert(
        "Access-Control-Expose-Headers",
        HeaderValue::from_static("X-Conversation-Id"),
    );

    Ok(res)
}
