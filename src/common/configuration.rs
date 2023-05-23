use serde::{Serialize, Deserialize};
use crate::common::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ThermometerWireConfig {
    pub id: u32,
    pub port: u32,
    pub name: String,
    pub is_port_valid: bool
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Configuration {
    pub thermometers: Vec<ThermometerWireConfig>,
    // pub mashControllers: Vec<MashControllerConfig>,
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

    pub fn clear_thermometers(&mut self) {
        self.thermometers.clear();
    }

}
