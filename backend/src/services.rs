use std::env;
use std::vec::Vec;

use reqwest;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
// use futures::future::{ready, Ready};
use serde_json;

use crate::types::*;

const API_KEY: &str = env!("OPEN_WEATHER_MAP_API_KEY", "$OPEN_WEATHER_MAP_API_KEY is not set");
const API_BASE_URL: &str = "http://api.openweathermap.org";

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Yo!")
}


#[get("/locations/{name}")]
pub async fn locations_service(req: HttpRequest) -> impl Responder {
    let name: String = req.match_info().get("name").unwrap().parse().unwrap();
    match find_location(name) {
        Ok(locations) => {
            let body = serde_json::to_string(&locations).unwrap();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        },
        Err(err) => {
            println!("{}", err);
            HttpResponse::NotFound().body("No locations")
        }
    }
}


fn find_location(location_name: String) ->  Result<Vec<Location>, Box<dyn std::error::Error>> {
    let request: String = format!("{}/geo/1.0/direct?q={}&limit=5&appid={}", API_BASE_URL, location_name, API_KEY);
    println!("REQUEST: {}", request);

    let locations: Vec<Location> = reqwest::blocking::get(request)?.json::<Vec<Location>>()?;
    // println!("{:?}", geolocations);
    Ok(locations)
}


#[get("/forecast/{lat},{lon}")]
pub async fn forecast_service(req: HttpRequest) -> impl Responder {
    let latitude: f32 = req.match_info().get("lat").unwrap().parse().unwrap();
    let longitude: f32 = req.match_info().get("lon").unwrap().parse().unwrap();
    // HttpResponse::Ok().body(format!("all good {},{}", latitude, longitude))

    match get_forecast(latitude, longitude) {
        // Ok(forecast) => HttpResponse::Ok().body(format!("{:#?}", forecast)),
        Ok(forecast) => {
            let body = serde_json::to_string(&forecast).unwrap();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        },
        Err(err) => {
            println!("{}", err);
            HttpResponse::NotFound().body("Not found")
        }
    }
}


fn get_forecast(latitude: f32, longitude: f32) -> Result<Forecast, Box<dyn std::error::Error>> {
    let excludes = "current,minutely,hourly,alerts"; // all but daily
    let request: String = format!("{}/data/2.5/onecall?&lat={}&lon={}&exclude={}&appid={}",
                                  API_BASE_URL, latitude, longitude, excludes, API_KEY);
    println!("REQUEST: {}", request);
    let forecast: Forecast = reqwest::blocking::get(request)?.json::<Forecast>()?;
    Ok(forecast)
}




