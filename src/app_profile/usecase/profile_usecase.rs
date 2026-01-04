use uuid::Uuid;

use crate::{
    app_profile::{
        domain::profile_domain::Profile, repository::profile_repository::ProfileRepository,
    },
    response::error::ResponseError,
};

pub struct ProfileUsecase<R: ProfileRepository> {
    repo: R,
}

impl<R: ProfileRepository> ProfileUsecase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_profile(&self, id: &Uuid) -> Result<Profile, ResponseError> {
        let find_profile = self.repo.find_profile_by_id(id).await.map_err(|e| {
            tracing::error!(error = ?e, "failed find profile");
            return ResponseError::DatabaseError;
        })?;

        let profile = match find_profile {
            Some(p) => p,
            None => return Err(ResponseError::NotFound("profile not found".into())),
        };

        Ok(profile)
    }
}
