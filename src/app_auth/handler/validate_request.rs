use crate::{app_auth::handler::types::{LoginRequest, RegisterRequest}, response::error::ResponseError};

pub fn validate_login_request(req: &LoginRequest) -> Result<(), ResponseError> {
    if req.username == "" {
        return Err(ResponseError::BadRequest("username is required".into()))
    }
    if req.password == "" {
        return Err(ResponseError::BadRequest("password is required".into()))
    }

    Ok(())
}

pub fn validate_register_request(req: &RegisterRequest) -> Result<(), ResponseError> {
    if req.name == "" {
        return Err(ResponseError::BadRequest("name is required".into()))
    }
    if req.username == "" {
        return Err(ResponseError::BadRequest("username is required".into()))
    }
    if req.email == "" {
        return Err(ResponseError::BadRequest("email is required".into()))
    }
    if req.password == "" {
        return Err(ResponseError::BadRequest("password is required".into()))
    }
    if req.re_password == "" {
        return Err(ResponseError::BadRequest("re_password is required".into()))
    }
    if req.password != req.re_password {
        return Err(ResponseError::BadRequest("password and re_password are not match".into()))
    }

    Ok(())
}