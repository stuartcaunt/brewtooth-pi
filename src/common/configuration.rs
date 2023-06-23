use serde::{Serialize, Deserialize};
use crate::common::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PIDConfig {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub output_max: f32,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MashControllerConfig {
    pub heater_port: u8,
    pub agitator_port: u8,
    pub pid: PIDConfig,
    pub window_size_ms: u64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub mash_controller: MashControllerConfig,
}

impl Configuration {
    pub fn read(config_path: &str) -> Result<Configuration> {
        let file = std::fs::File::open("config.yml").expect("Could not open file.");
        
        match serde_yaml::from_reader(file) {
            Err(error) => return Err(BrewtoothError::ConfigError(format!("Failed to read config file {}: {}", config_path, error))),
            Ok(config) => {
                let configuration: Configuration = config;
                return Ok(configuration)
            }
        }
    }
}
