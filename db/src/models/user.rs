use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Store;
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
}

impl Store {
    pub async fn create_user(&self, username: String, password: String) -> Result<UserResponse> {
        info!("Creating a new user with username: {}", username);

        let user = sqlx::query_as!(
            UserResponse,
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id",
            username,
            password
        )
        .fetch_one(&self.pool)
        .await?;

        info!("User created successfully with id: {}", user.id);

        Ok(UserResponse {
            id: user.id.to_string(),
        })
    }
}
