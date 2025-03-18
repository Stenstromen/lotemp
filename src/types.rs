use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub current_units: CurrentUnits,
    pub current: Current,
}

#[derive(Deserialize, Debug)]
pub struct CurrentUnits {
    pub temperature_2m: String,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    pub time: String,
    pub temperature_2m: f64,
}

#[derive(Deserialize, Debug)]
pub struct GeoLocationResponse {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub latitude: f64,
    pub longitude: f64,
}
