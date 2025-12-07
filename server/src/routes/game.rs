use actix_web::{
    HttpResponse,
    web::{Data, Json, Path, Query, ReqData},
};

use db::{
    Store,
    models::game::{ChangeGameStatusResponse, Game, Status},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateGame {
    pub room_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetGamePath {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetGameQuery {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeGameStatus {
    pub status: Status,
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
    let uuid = Uuid::parse_str(&claims.into_inner())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid UUID"))?;

    let game = store
        .create_game(&req_body.room_name, &uuid)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(game))
}

pub async fn get_game(
    store: Data<Store>,
    path: Option<Path<GetGamePath>>,
    query: Option<Query<GetGameQuery>>, // req_body: Json<GetGame>,
) -> Result<Json<Game>, actix_web::error::Error> {
    let game_id_from_req = if let Some(p) = path {
        p.into_inner().id
    } else if let Some(q) = query {
        q.into_inner().id
    } else {
        Err(actix_web::error::ErrorBadRequest("Missing game id"))?
    };

    let game_id = Uuid::parse_str(&game_id_from_req)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid UUID"))?;

    let game = store
        .get_game_by_id(&game_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(Json(game))
}
pub async fn change_game_status(
    store: Data<Store>,
    path: Option<Path<GetGamePath>>,
    query: Option<Query<GetGameQuery>>,
    req_body: Json<ChangeGameStatus>,
) -> Result<Json<ChangeGameStatusResponse>, actix_web::error::Error> {
    let game_id_from_req = if let Some(p) = path {
        p.into_inner().id
    } else if let Some(q) = query {
        q.into_inner().id
    } else {
        Err(actix_web::error::ErrorBadRequest("Missing game id"))?
    };

    let game_id = Uuid::parse_str(&game_id_from_req)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid UUID"))?;

    let game = store
        .change_game_status_by_id(&game_id, &req_body.status)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(Json(game))
}
