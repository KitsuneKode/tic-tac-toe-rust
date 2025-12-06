use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::Store;
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: String,
}

impl Store {
    pub async fn create_user(
        &self,
        username: &String,
        password: &String,
    ) -> Result<CreateUserResponse> {
        info!("Creating a new user with username: {}", username);

        let now = Utc::now();
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password, updated_at) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(username)
        .bind(password)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        info!("User created successfully with id: {}", user.id);
        Ok(CreateUserResponse {
            id: user.id.to_string(),
        })
    }

    pub async fn get_user_by_username(&self, username: &String) -> Result<User> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username=$1")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;

        info!("User with username {} not found", &username);

        Ok(user)
    }

    pub async fn get_user_by_uuid(&self, uuid: &Uuid) -> Result<User> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id=$1")
            .bind(uuid)
            .fetch_one(&self.pool)
            .await?;

        info!("User with id {} not found", &uuid);

        Ok(user)
    }
}
