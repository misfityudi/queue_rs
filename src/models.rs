use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Blog {
#[serde(default)] // Make `id` optional for POST/PUT requests
    pub id: uuid::Uuid,
    pub title: String,
    pub published: bool,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
#[serde(default)] // Make `id` optional for POST/PUT requests
    pub id: uuid::Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Profile {
#[serde(default)] // Make `id` optional for POST/PUT requests
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub bio: String,
    pub avatar_url: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Session {
#[serde(default)] // Make `id` optional for POST/PUT requests
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub expires_at: chrono::NaiveDateTime,
}