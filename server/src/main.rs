use anyhow::Result;

use actix_web::{App, HttpServer, web};

use dotenv::dotenv;

pub mod routes;
use routes::user::{echo, hello};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let store = db::Store::new().await?;

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api/v1"))
            .service(hello)
            .service(echo)
            .app_data(web::Data::new(store.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
