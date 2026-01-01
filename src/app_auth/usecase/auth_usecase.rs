use uuid::Uuid;

use crate::app_auth::domain::auth_domain::User;
use crate::app_auth::usecase::helper::{hash_password, verify_password};
use crate::app_auth::{
    repository::auth_repository::AuthRepository, usecase::types::AuthUsecaseError,
};
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
                return Err(AuthUsecaseError::InternalServerError);
            }
        };
        if !is_password_match {
            return Err(AuthUsecaseError::WrongUsernameOrPassword);
        }

        let generate_token = generate_token(&user.id.to_string());
        let token = match generate_token {
            Ok(token) => token,
            Err(e) => {
                tracing::error!(
                    error = ?e,
                    "generate_token error"
                );
                return Err(AuthUsecaseError::InternalServerError);
            }
        };

        Ok(token)
    }

    pub async fn register(
        &self,
        name: &str,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(), AuthUsecaseError> {
        // CHECK EMAIL
        let check_email = self
        .repo
        .check_existing_user_by_email(email, None)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed check email");
            return AuthUsecaseError::DatabaseError;
        })?;
        if check_email {
            return Err(AuthUsecaseError::EmailAlreadyExist);
        }

        // CHECK USERNAME
        let check_username = self
        .repo
        .check_existing_user_by_username(username, None)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed check username");
            return AuthUsecaseError::DatabaseError;
        })?;
        if check_username {
            return Err(AuthUsecaseError::UsernameAlreadyExist);
        }

        // ENCRYPT PASSWORD
        let encrypted_password = hash_password(password).map_err(|e| {
            tracing::error!(error = ?e, "failed generate password");
            return AuthUsecaseError::InternalServerError;
        })?;

        // PREPARE USER DATA
        let user: User = User {
            id: Uuid::new_v4(),
            name: name.to_string(),
            username: username.to_string(),
            email: email.to_string(),
            encrypted_password: encrypted_password,
            phone_number: None,
            photo_profile: None,
            token_version: 0.into(),
        };

        // STORE USER DATA
        self
        .repo
        .create_user(&user)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed store data to database");
            return AuthUsecaseError::InternalServerError;
        })?;

        Ok(())
    }
}
