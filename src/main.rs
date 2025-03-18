mod api;
mod config;
mod types;
use api::{fetch_current_temperature, fetch_geo_location};
use config::{read_config, store_config};
use std::env;
use std::error::Error;
use types::{AppConfig, GeoLocationResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "init" {
        let geo_location: GeoLocationResponse = fetch_geo_location().await?;
        let config: AppConfig = AppConfig {
            latitude: geo_location.latitude,
            longitude: geo_location.longitude,
        };

        store_config(&config)?;

        println!("Initialization complete. Current GeoIP location stored in ~/.lotemp");
    } else {
        let config: AppConfig = read_config()?;
        let current_temperature: String = fetch_current_temperature(config).await?;
        println!("{}", current_temperature);
    }
    Ok(())
}
