use std::collections::HashMap;
use std::vec::Vec;
use std::env;
use reqwest;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
struct GeoCoding {
    name: String,
    local_names: HashMap<String, String>,
    lat: f64,
    lon: f64,
    country: String,
    state: String
}

// struct Location {
//     longitude: f32,
//     latitude: f32,
// }

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_location)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/locations/{name}")]
async fn get_location(req: HttpRequest) -> impl Responder {
    let name: String = req.match_info().get("name").unwrap().parse().unwrap();
    HttpResponse::Ok().body(format!("Hello yo from {}!", name))
}

fn find_location(location: &str) ->  Result<(), Box<dyn std::error::Error>> {
    const API_KEY: &str = env!("OPEN_WEATHER_MAP_API_KEY", "$OPEN_WEATHER_MAP_API_KEY is not set");

    const API_URL: &str = "http://api.openweathermap.org/geo/1.0/direct?q=";
    let request = format!("{}{}&appid={}", API_URL, location, API_KEY);
    println!("REQUEST: {}", request);

    let resp = reqwest::blocking::get(request)?.json::<Vec<GeoCoding>>();
    println!("{:?}", resp);
    Ok(())
}
