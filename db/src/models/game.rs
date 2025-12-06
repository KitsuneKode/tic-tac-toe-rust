use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use tracing::info;
use uuid::Uuid;

use crate::Store;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "game_status", rename_all = "UPPERCASE")]
pub enum Status {
    WIN,
    LOSS,
    NOT_STARTED,
    DRAW,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub room_name: String,
    pub winner_id: Option<Uuid>,
    pub status: Status,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameResponse {
    pub id: String,
    pub status: Status,
}

impl Store {
    pub async fn create_game(&self, room_name: &String) -> Result<CreateGameResponse> {
        let game =
            sqlx::query_as::<_, Game>("INSERT INTO games (room_name) VALUES ($1) RETURNING *")
                .bind(room_name)
                .fetch_one(&self.pool)
                .await?;

        info!("Created Game with name {}", &room_name);
        Ok(CreateGameResponse {
            id: game.id.to_string(),
            status: game.status,
        })
    }
}
