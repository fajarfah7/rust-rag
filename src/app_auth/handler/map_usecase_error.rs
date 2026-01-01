use crate::{app_auth::usecase::types::AuthUsecaseError, response::error::ResponseError};

pub fn map_usecase_auth_error(err: AuthUsecaseError) -> ResponseError {
    match err {
        AuthUsecaseError::WrongUsernameOrPassword => {
            ResponseError::BadRequest("wrong username or password".into())
        }
        AuthUsecaseError::DatabaseError => ResponseError::DatabaseError,
        AuthUsecaseError::InternalServerError => ResponseError::InternalServerError,
        AuthUsecaseError::EmailAlreadyExist => {
            ResponseError::BadRequest("email already exist".into())
        }
        AuthUsecaseError::UsernameAlreadyExist => {
            ResponseError::BadRequest("username already exist".into())
        }
    }
}
