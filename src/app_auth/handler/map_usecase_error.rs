use crate::{app_auth::usecase::types::AuthUsecaseError, response::error::ResponseError};

pub fn map_usecase_auth_error(err: AuthUsecaseError) -> ResponseError {
    match err {
        AuthUsecaseError::WrongUsernameOrPassword => {
            ResponseError::BadRequest("wrong username or password".into())
        }
        AuthUsecaseError::DatabaseError => {
            ResponseError::DatabaseError
        }
        AuthUsecaseError::InternalServerError => {
            ResponseError::InternalServerError
        }
    }
}