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

    async fn check_existing_user_by_email(
        &self, 
        email: &str, 
        id: Option<&Uuid>
    ) -> Result<bool, sqlx::Error> {
        let is_exist = match id {
            Some(id) => {
                sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS (
                        SELECT 1 
                        FROM users
                        WHERE email = $1 AND id != $2
                    )
                    "#,
                    email,
                    id
                )
                .fetch_one(&self.pool)
                .await?
            }
            None => {
                sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS (
                        SELECT 1 
                        FROM users
                        WHERE email = $1
                    )
                    "#,
                    email
                )
                .fetch_one(&self.pool)
                .await?
            }
        };

        Ok(is_exist.unwrap_or(false))
    }

    async fn check_existing_user_by_username(
        &self, 
        username: &str, 
        id: Option<&Uuid>
    ) -> Result<bool, sqlx::Error> {
        let is_exist = match id {
            Some(id) => {
                sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS (
                        SELECT 1 
                        FROM users
                        WHERE username = $1 AND id != $2
                    )
                    "#,
                    username,
                    id
                )
                .fetch_one(&self.pool)
                .await?
            }
            None => {
                sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS (
                        SELECT 1 
                        FROM users
                        WHERE username = $1
                    )
                    "#,
                    username
                )
                .fetch_one(&self.pool)
                .await?
            }
        };

        Ok(is_exist.unwrap_or(false))
    }

    async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO users(
                id,
                name,
                username,
                email,
                encrypted_password,
                phone_number,
                photo_profile,
                token_version
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            user.id,
            user.name,
            user.username,
            user.email,
            user.encrypted_password,
            user.phone_number,
            user.photo_profile,
            user.token_version,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}