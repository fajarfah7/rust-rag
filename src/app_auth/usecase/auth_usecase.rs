
use crate::app_auth::{
    repository::auth_repository::AuthRepository, usecase::types::AuthUsecaseError,
};
use crate::app_auth::usecase::helper::{verify_password};
use crate::middleware::jwt_token::jwt::generate_token;

pub struct AuthUsecase<R: AuthRepository> {
    repo: R,
}

impl<R: AuthRepository> AuthUsecase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
    pub async fn login(&self, username: &str, password: &str) -> Result<String, AuthUsecaseError> {
        let some_user = self
            .repo
            .find_user_by_username(username)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    "storage error"
                );
                AuthUsecaseError::DatabaseError
            })?;

        let user = match some_user {
            Some(user) => user,
            _ => return Err(AuthUsecaseError::WrongUsernameOrPassword),
        };

        let verify_password = verify_password(password, &user.encrypted_password);
        let is_password_match = match verify_password {
            Ok(is_match) => is_match,
            Err(e) => {
                tracing::error!(
                    error = ?e,
                    "verify_password error"
                );
                return Err(AuthUsecaseError::InternalServerError)
            }
        };
        if !is_password_match {
            return Err(AuthUsecaseError::WrongUsernameOrPassword)
        }

        let generate_token = generate_token(&user.id.to_string());
        let token = match generate_token {
            Ok(token) => token,
            Err(e) => {
                tracing::error!(
                    error = ?e,
                    "generate_token error"
                );
                return Err(AuthUsecaseError::InternalServerError)
            }
        };

        Ok(token)
    }
}
