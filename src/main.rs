mod api;
mod config;
mod types;
use api::{fetch_current_temperature, fetch_geo_location};
use config::{read_config, store_config};
use std::env;
use std::error::Error;
use types::{AppConfig, GeoLocationResponse};

fn get_version() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "-v" || args[1] == "--version") {
        println!("{}", get_version());
        return Ok(());
    }

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
        println!("{current_temperature}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_format() {
        let version = get_version();
        let expected = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        assert_eq!(version, expected);
    }

    #[test]
    fn test_version_contains_package_name() {
        let version = get_version();
        assert!(version.contains(env!("CARGO_PKG_NAME")));
    }

    #[test]
    fn test_version_contains_version_number() {
        let version = get_version();
        assert!(version.contains(env!("CARGO_PKG_VERSION")));
    }
}
