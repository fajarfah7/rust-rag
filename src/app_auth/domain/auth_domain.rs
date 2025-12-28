use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub encrypted_password: String,
    pub phone_number: Option<String>,
    pub photo_profile: Option<String>,
    pub token_version: Option<i32>,
}