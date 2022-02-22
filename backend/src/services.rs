use std::env;
use std::vec::Vec;

use reqwest;
use actix_web::{get, HttpRequest, HttpResponse, Responder};

use crate::types::*;

const API_KEY: &str = env!("OPEN_WEATHER_MAP_API_KEY", "$OPEN_WEATHER_MAP_API_KEY is not set");
const API_URL: &str = "http://api.openweathermap.org/geo/1.0/direct?q=";

#[get("/locations/{name}")]
pub async fn get_location(req: HttpRequest) -> impl Responder {
    let name: String = req.match_info().get("name").unwrap().parse().unwrap();
    match find_location(name) {
        Ok(locations) => HttpResponse::Ok().body(format!("{:#?}", locations)),
        Err(err) => {
            println!("{}", err);
            HttpResponse::NotFound().body("No locations")
        }
    }

}

fn find_location(location: String) ->  Result<Vec<Location>, Box<dyn std::error::Error>> {
    let request = format!("{}{}&limit=5&appid={}", API_URL, location, API_KEY);
    println!("REQUEST: {}", request);

    let locations: Vec<Location> = reqwest::blocking::get(request)?.json::<Vec<Location>>()?;
    // println!("{:?}", geolocations);
    Ok(locations)
}

