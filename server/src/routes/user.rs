use actix_web::web::{Data, Json, ReqData};
use chrono::Utc;
use db::Store;
use jsonwebtoken::{EncodingKey, Header, encode};
use log::info;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SignUpUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpResponse {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct MeHandlerResponse {
    id: Uuid,
    username: String,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    fn new(user_id: String) -> Self {
        Self {
            sub: user_id,
            exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
        }
    }
}

pub async fn sign_up(
    store: Data<Store>,
    req_body: Json<SignUpUser>,
) -> Result<Json<SignUpResponse>, actix_web::error::Error> {
    let user = store
        .create_user(&req_body.username, &req_body.password)
        .await
        .map_err(|e| actix_web::error::ErrorConflict(e.to_string()))?;

    Ok(Json(SignUpResponse { id: user.id }))
}

pub async fn sign_in(
    store: Data<Store>,
    req_body: Json<SignInUser>,
) -> Result<Json<SignInResponse>, actix_web::error::Error> {
    let user = store
        .get_user_by_username(&req_body.username)
        .await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

    if user.password != req_body.password {
        return Err(actix_web::error::ErrorUnauthorized("Incorrect password"));
    }

    let token = encode(
        &Header::default(),
        &Claims::new(user.id.to_string()),
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?
                .as_bytes(),
        ),
    )
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(Json(SignInResponse { token }))
}

pub async fn get_user_data(
    store: Data<Store>,
    claims: ReqData<String>, // <-- this pulls from extensions
) -> Result<Json<MeHandlerResponse>, actix_web::error::Error> {
    let uuid = Uuid::parse_str(&claims.into_inner()).unwrap();

    info!("uuid is {}", uuid);

    let user = store
        .get_user_by_uuid(&uuid)
        .await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

    Ok(Json(MeHandlerResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}
