use crate::types::{ApiResponse, AppConfig, GeoLocationResponse};
use regex::Regex;
use std::error::Error;

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

    let result: Result<GeoLocationResponse, Box<dyn Error>> = match (latitude, longitude) {
        (Some(lat), Some(lon)) => Ok(GeoLocationResponse {
            latitude: lat,
            longitude: lon,
        }),
        _ => Err("Failed to parse latitude and longitude from response".into()),
    };

    result
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

    Ok(format!(
        "{} - {}{}",
        formatted_time,
        api_response.current.temperature_2m,
        api_response.current_units.temperature_2m
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_location_parsing() {
        let test_response = "IP: ---\n\
            Country: France\n\
            Country (ISO code): FR\n\
            Is EU?: true\n\
            State: Upper France\n\
            State Code:\n\
            Zip Code: 59100\n\
            Country Capital: Paris\n\
            Latitude: 50.69224\n\
            Longitude: 3.20004\n\
            ISP: ---\n\
            Timezone: Europe/Paris";

        let mut latitude: Option<f64> = None;
        let mut longitude: Option<f64> = None;

        for line in test_response.lines() {
            if line.starts_with("Latitude:") {
                latitude = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|lat_str| lat_str.parse().ok());
            } else if line.starts_with("Longitude:") {
                longitude = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|lon_str| lon_str.parse().ok());
            }
        }

        let result: Result<GeoLocationResponse, Box<dyn Error>> = match (latitude, longitude) {
            (Some(lat), Some(lon)) => Ok(GeoLocationResponse {
                latitude: lat,
                longitude: lon,
            }),
            _ => Err("Failed to parse latitude and longitude from response".into()),
        };

        assert!(result.is_ok());
        let location = result.unwrap();
        assert_eq!(location.latitude, 50.69224);
        assert_eq!(location.longitude, 3.20004);
    }

    #[test]
    fn test_temperature_formatting() {
        let test_response = r#"{
            "latitude": 50.692,
            "longitude": 3.1899998,
            "generationtime_ms": 0.01621246337890625,
            "utc_offset_seconds": 3600,
            "timezone": "Europe/Paris",
            "timezone_abbreviation": "GMT+1",
            "elevation": 24.0,
            "current_units": {
                "time": "iso8601",
                "interval": "seconds",
                "temperature_2m": "°C"
            },
            "current": {
                "time": "2025-03-18T15:00",
                "interval": 900,
                "temperature_2m": 11.9
            }
        }"#;

        let api_response: ApiResponse = serde_json::from_str(test_response).unwrap();
        let time_with_t = &api_response.current.time;
        let re = Regex::new("T").unwrap();
        let formatted_time = re.replace_all(time_with_t, " ");

        let formatted_output = format!(
            "{} - {}{}",
            formatted_time,
            api_response.current.temperature_2m,
            api_response.current_units.temperature_2m
        );

        assert_eq!(formatted_output, "2025-03-18 15:00 - 11.9°C");
    }

    #[test]
    fn test_invalid_geo_location() {
        let test_response = "Invalid data\nNo coordinates here\n";
        let mut latitude: Option<f64> = None;
        let mut longitude: Option<f64> = None;

        for line in test_response.lines() {
            if line.starts_with("Latitude:") {
                latitude = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|lat_str| lat_str.parse().ok());
            } else if line.starts_with("Longitude:") {
                longitude = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|lon_str| lon_str.parse().ok());
            }
        }

        let result: Result<GeoLocationResponse, Box<dyn Error>> = match (latitude, longitude) {
            (Some(lat), Some(lon)) => Ok(GeoLocationResponse {
                latitude: lat,
                longitude: lon,
            }),
            _ => Err("Failed to parse latitude and longitude from response".into()),
        };

        assert!(result.is_err());
    }
}
