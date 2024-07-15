mod types;
mod config;
mod api;
use config::{ store_config, read_config };
use types::{ AppConfig, GeoLocationResponse };
use api::{ fetch_geo_location, fetch_current_temperature };
use std::env;
use std::error::Error;

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
