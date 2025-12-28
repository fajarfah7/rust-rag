use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct ResponseSuccessBody<T> {
    message: String,
    http_code: u16,
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<PaginationMeta>,
}

#[derive(Serialize, Debug)]
struct PaginationMeta {
    page: u32,
    per_page: u32,
    total_data: u64,
    total_page: u32,
}

#[derive(Debug)]
pub enum ResponseSuccess<T> {
    NoData(StatusCode),
    Object(StatusCode, Option<T>),
    Pagination(u32, u32, u64, Option<T>),
}

impl<T> IntoResponse for ResponseSuccess<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            ResponseSuccess::NoData(status) => (status, {
                let body: ResponseSuccessBody<T> = ResponseSuccessBody {
                    message: "success".into(),
                    http_code: status.as_u16(),
                    data: None,
                    meta: None,
                };
                Json(body)
            })
                .into_response(),
            ResponseSuccess::Object(status, data) => (status, {
                let body: ResponseSuccessBody<T> = ResponseSuccessBody {
                    message: "success".into(),
                    http_code: status.as_u16(),
                    data: data,
                    meta: None,
                };
                Json(body)
            })
                .into_response(),
            ResponseSuccess::Pagination(page, per_page, total_data, data) => (StatusCode::OK, {
                // let total_page: u32 = (total_company as f64 / (query.per_page.unwrap_or(1) as f64)).ceil() as u32;
                let total_page = (total_data as f64 / per_page as f64).ceil() as u32;
                let meta: PaginationMeta = PaginationMeta {
                    page,
                    per_page,
                    total_data,
                    total_page,
                };
                let body: ResponseSuccessBody<T> = ResponseSuccessBody {
                    message: "success".into(),
                    http_code: StatusCode::OK.as_u16(),
                    data: data,
                    meta: Some(meta),
                };
                Json(body)
            })
                .into_response(),
        }
    }
}

// // IF WANT TO IMPLEMENT STRUCT FOR RESPONSE SUCCESS
// impl<T> ResponseSuccessBody<T>
// where
//     T: Serialize,
// {
//     pub fn default(status: StatusCode, data: Option<T>) -> impl IntoResponse {
//         let body: ResponseSuccessBody<T> = ResponseSuccessBody {
//             message: "success".into(),
//             http_code: status.as_u16(),
//             data: data,
//             meta: None,
//         };
//         (status, Json(body))
//     }

//     pub fn pagination(
//         page: u32,
//         per_page: u32,
//         total_data: u64,
//         data: Option<T>,
//     ) -> impl IntoResponse {
//         let status = StatusCode::OK;

//         let total_page = ((total_data + per_page as u64 - 1) / per_page as u64) as u32;

//         let body: ResponseSuccessBody<T> = ResponseSuccessBody {
//             message: "success".into(),
//             http_code: status.as_u16(),
//             data: data,
//             meta: Some(PaginationMeta {
//                 page: page,
//                 per_page: per_page,
//                 total_data: total_data,
//                 total_page: total_page,
//             }),
//         };

//         (status, Json(body))
//     }
// }
