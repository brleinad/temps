use std::collections::HashMap;
use std::vec::Vec;
use std::env;
use reqwest;
use tokio;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const API_KEY: &str = env!("OPEN_WEATHER_MAP_API_KEY", "$OPEN_WEATHER_MAP_API_KEY is not set");

    const API_URL: &str = "http://api.openweathermap.org/geo/1.0/direct?q=";
    let city_name = "Guatemala";
    let request = format!("{}{}&appid={}", API_URL, city_name, API_KEY);
    println!("REQUEST: {}", request);

    let resp = reqwest::get(request)
        .await?
        .json::<Vec<GeoCoding>>()
        .await?;
    println!("{:?}", resp);
    Ok(())
}
