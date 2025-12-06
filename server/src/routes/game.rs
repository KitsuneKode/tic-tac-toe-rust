use actix_web::{
    HttpMessage, HttpResponse,
    web::{Data, Json},
};

use db::{Store, models::game::Status};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateGame {
    pub room_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameResponse {
    id: String,
    status: Status,
}

pub async fn create_game(
    store: Data<Store>,
    req_body: Json<CreateGame>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let game = store
        .create_game(&req_body.room_name)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(game))
}
