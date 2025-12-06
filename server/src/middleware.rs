use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use log::{error, info};
use std::env;

use crate::routes::user::Claims;

#[derive(Debug, Clone)]
pub struct JwtClaims(pub Claims);

pub async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    if let Some(header_value) = req.headers().get("Authorization") {
        let token = match header_value.to_str() {
            Ok(t) => t,
            Err(_) => return Err(ErrorUnauthorized("Invalid Authorization header")),
        };

        let secret = env::var("JWT_SECRET").expect("SECRET_KEY must be set");
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::default();

        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                info!("Extension data get {}", token_data.claims.sub);
                req.extensions_mut().insert(token_data.claims.sub);
            }
            Err(e) => {
                error!("JWT decoding error: {:?}", e);
                return Err(ErrorUnauthorized("Invalid JWT token"));
            }
        }
    }
    // Must return the service response
    next.call(req).await
}
