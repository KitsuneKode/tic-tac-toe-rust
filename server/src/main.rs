use actix_web::middleware::{Logger, from_fn};
use actix_web::web::{self, get, post};
use actix_web::{App, HttpServer};
use anyhow::Result;
use dotenvy::dotenv;
use env_logger::Env;
use std::env;

use crate::middleware::my_middleware;
use crate::routes::user::{get_user_data, sign_in, sign_up};

use crate::utils::{get_health, not_found};

pub mod middleware;
pub mod routes;
pub mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let store = db::Store::new().await?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/health", get().to(get_health))
            .service(
                web::scope("/api/v1")
                    .service(web::resource("/signin").route(post().to(sign_in)))
                    .service(web::resource("/signup").route(post().to(sign_up)))
                    .wrap(from_fn(my_middleware))
                    .service(web::resource("/data").route(get().to(get_user_data)))
                    .app_data(web::Data::new(store.clone())),
            )
            .default_service(web::route().to(not_found))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}
