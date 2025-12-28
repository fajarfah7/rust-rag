use crate::{app_auth::handler::types::LoginRequest, response::error::ResponseError};

pub fn validate_request(req: &LoginRequest) -> Result<(), ResponseError> {
    if req.username == "" {
        return Err(ResponseError::BadRequest("username is required".into()))
    }
    if req.password == "" {
        return Err(ResponseError::BadRequest("password is required".into()))
    }

    Ok(())
}