use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::app_auth::{domain::auth_domain::User, repository::auth_repository::AuthRepository};

pub struct AuthRepositorySqlx {
    pool: PgPool
}

impl AuthRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositorySqlx {
    async fn find_user_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<User>, sqlx::Error> {
        let user: User = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, username, email, encrypted_password, phone_number, photo_profile, token_version
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Some(user))
    }

    async fn find_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user: User = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, username, email, encrypted_password, phone_number, photo_profile, token_version
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(user))
    }
}