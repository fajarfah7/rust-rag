use axum::async_trait;
use uuid::Uuid;

use crate::app_profile::domain::profile_domain::Profile;

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn find_profile_by_id(&self, id: &Uuid) -> Result<Option<Profile>, sqlx::Error>;
}