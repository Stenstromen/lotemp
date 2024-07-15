use crate::types::{ ApiResponse, AppConfig, GeoLocationResponse };
use std::error::Error;
use regex::Regex;

pub async fn fetch_geo_location() -> Result<GeoLocationResponse, Box<dyn Error>> {
    let api_url: &str = "https://addr.se/geo";
    let response_text: String = reqwest::get(api_url).await?.text().await?;
    let mut latitude: Option<f64> = None;
    let mut longitude: Option<f64> = None;

    for line in response_text.lines() {
        if line.starts_with("Latitude:") {
            latitude = line
                .split_whitespace()
                .nth(1)
                .and_then(|lat_str: &str| lat_str.parse().ok());
        } else if line.starts_with("Longitude:") {
            longitude = line
                .split_whitespace()
                .nth(1)
                .and_then(|lon_str: &str| lon_str.parse().ok());
        }
    }

    match (latitude, longitude) {
        (Some(lat), Some(lon)) => Ok(GeoLocationResponse { latitude: lat, longitude: lon }),
        _ => Err("Failed to parse latitude and longitude from response".into()),
    }
}

pub async fn fetch_current_temperature(config: AppConfig) -> Result<String, Box<dyn Error>> {
    let request_url: String = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current=temperature_2m&timezone=auto",
        latitude = config.latitude,
        longitude = config.longitude
    );
    let response: reqwest::Response = reqwest::get(&request_url).await?;
    let api_response: ApiResponse = response.json().await?;
    let time_with_t: &String = &api_response.current.time;
    let re: Regex = Regex::new("T").unwrap();
    let formatted_time: std::borrow::Cow<str> = re.replace_all(time_with_t, " ");

    Ok(
        format!(
            "{} - {}{}",
            formatted_time,
            api_response.current.temperature_2m,
            api_response.current_units.temperature_2m
        )
    )
}
