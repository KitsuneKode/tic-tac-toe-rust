use actix_web::{HttpResponse, web::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn get_health() -> Result<Json<HealthResponse>, actix_web::error::Error> {
    Ok(Json(HealthResponse {
        status: String::from("OK"),
    }))
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Page not found")
}
