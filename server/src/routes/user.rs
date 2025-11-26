use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder, get, post};
use db::Store;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    username: String,
    password: String,
}

#[get("/user")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/user")]
async fn echo(state: Data<Store>, req_body: Json<CreateUser>) -> impl Responder {
    let store = state.into_inner();
    let user = store
        .create_user(username:req_body.into_inner().username , password: req_body.into_inner().password)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    Ok(Json(json!(user))
}
