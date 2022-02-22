use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    name: String,
    lat: f32,
    lon: f32,
    country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    lat: f32,
    lon: f32,
    timezone: String,
    timezone_offset:  f32,
    daily: Vec<DailyForecast>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DailyForecast {
    humidity: u32,
    weather: Vec<Weather>,
    rain: f32,
    temp: Temp,
    // feels_like: Temp TODO: just like temp but without min or max
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String, // TODO enum?
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Temp {
    day: f32,
    min: f32,
    max: f32,
    night: f32,
    eve: f32,
    morn: f32,
}
