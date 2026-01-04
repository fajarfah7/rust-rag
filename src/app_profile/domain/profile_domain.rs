use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub photo_profile: Option<String>,
    pub phone_number: Option<String>,
}

