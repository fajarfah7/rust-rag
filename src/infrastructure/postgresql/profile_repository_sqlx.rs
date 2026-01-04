use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::app_profile::{domain::profile_domain::Profile, repository::profile_repository::ProfileRepository};

pub struct ProfileRepositorySqlx{
    pool: PgPool,
}

impl ProfileRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfileRepository for ProfileRepositorySqlx {
    async fn find_profile_by_id(&self, id: &Uuid) -> Result<Option<Profile>, sqlx::Error> {
        let profile = sqlx::query_as!(
            Profile,
            r#"
            SELECT id, name, username, email, photo_profile, phone_number
            FROM users
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(profile))
    }
}