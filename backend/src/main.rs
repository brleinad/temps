
use actix_web::{App, HttpServer};

mod types;
mod services;

use services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(locations_service)
            .service(forecast_service)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
