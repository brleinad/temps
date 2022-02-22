use actix_web::{App, HttpServer};
use actix_web::http::header;
use actix_cors::Cors;

mod types;
mod services;

use services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://www.temps-frontend.render.com")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(locations_service)
            .service(forecast_service)
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
