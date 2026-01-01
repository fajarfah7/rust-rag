use axum::async_trait;
use uuid::Uuid;

use crate::app_auth::domain::auth_domain::User;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_user_by_id(&self, id: &Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error>;
    async fn check_existing_user_by_email(&self, email: &str, id: Option<&Uuid>) -> Result<bool, sqlx::Error>;
    async fn check_existing_user_by_username(&self, username: &str, id: Option<&Uuid>) -> Result<bool, sqlx::Error>;
    async fn create_user(&self, user: &User) -> Result<(), sqlx::Error>;
}