use crate::types::AppConfig;
use std::env::var;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::fs::{ read_to_string, write };
use serde_yaml::{ to_string, from_str };

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
