use crate::types::AppConfig;
use serde_yaml::{from_str, to_string};
use std::env::var;
use std::error::Error;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::process::exit;

pub fn store_config(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    let home_dir: String = var("HOME")?;
    let config_path: PathBuf = PathBuf::from(home_dir).join(".lotemp");
    let config_str: String = to_string(config)?;
    write(config_path, config_str)?;
    Ok(())
}

pub fn read_config() -> Result<AppConfig, Box<dyn Error>> {
    let home_dir: String = var("HOME")?;
    let config_path: PathBuf = PathBuf::from(home_dir).join(".lotemp");

    if !config_path.exists() {
        println!("!!! Config file not found. Please run: lotemp init\n");
        exit(1);
    }

    let config_str: String = read_to_string(config_path)?;
    let config: AppConfig = from_str(&config_str)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_config_file() -> Result<(), Box<dyn Error>> {
        // Create a temporary directory for our test
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join(".lotemp");

        // Create test config content
        let test_config = r#"---
latitude: 50.692
longitude: 3.1899998"#;

        // Write test config to temporary file
        write(&config_path, test_config)?;

        // Override HOME environment variable for this test
        temp_env::with_var("HOME", Some(temp_dir.path().to_str().unwrap()), || {
            // Read and parse the config
            let config = read_config().unwrap();

            // Verify the values
            assert_eq!(config.latitude, 50.692);
            assert_eq!(config.longitude, 3.1899998);
        });

        Ok(())
    }
}
