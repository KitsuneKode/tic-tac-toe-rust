use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use tracing::info;
use uuid::Uuid;

use crate::Store;

#[allow(non_camel_case_types)]
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
    pub created_by: Uuid,
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

#[derive(Serialize, Deserialize)]
pub struct ChangeGameStatusResponse {
    pub id: String,
}

impl Store {
    pub async fn create_game(
        &self,
        room_name: &String,
        created_by: &Uuid,
    ) -> Result<CreateGameResponse> {
        let game = sqlx::query_as::<_, Game>(
            "INSERT INTO games (room_name, created_by, status) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(room_name)
        .bind(created_by)
        .bind(Status::NOT_STARTED)
        .fetch_one(&self.pool)
        .await?;

        info!("Created Game with name {}", &room_name);
        Ok(CreateGameResponse {
            id: game.id.to_string(),
            status: game.status,
        })
    }

    pub async fn get_game_by_id(&self, game_id: &Uuid) -> Result<Game> {
        let game = sqlx::query_as::<_, Game>("SELECT * from games WHERE id=$1")
            .bind(game_id)
            .fetch_one(&self.pool)
            .await?;
        info!("Game found with id {}", &game_id);

        Ok(game)
    }
    pub async fn change_game_status_by_id(
        &self,
        game_id: &Uuid,
        status: &Status,
    ) -> Result<ChangeGameStatusResponse> {
        let game = sqlx::query_as::<_, Game>("UPDATE games SET status=$2 WHERE id=$1 RETURNING *")
            .bind(game_id)
            .bind(status)
            .fetch_one(&self.pool)
            .await?;
        info!("Game found with id {}", &game_id);

        Ok(ChangeGameStatusResponse {
            id: game.id.to_string(),
        })
    }
}
