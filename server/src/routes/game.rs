use actix_web::{
    HttpResponse,
    web::{Data, Json, ReqData},
};

use db::{Store, models::game::Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    claims: ReqData<String>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let uuid = Uuid::parse_str(&claims.into_inner()).unwrap();

    let game = store
        .create_game(&req_body.room_name, &uuid)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(game))
}
