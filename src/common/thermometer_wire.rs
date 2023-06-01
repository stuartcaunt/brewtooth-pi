use std::fs;
use crate::common::{Thermometer};
use crate::common::*;

pub struct ThermometerWire {
    thermometers: Vec<Thermometer>
}

impl ThermometerWire {

    pub fn new() -> Result<Self> {
        let thermometers = ThermometerWire::search()?;
        Ok(Self {
            thermometers: thermometers
        })
    }

    pub fn get_temperature(&self) -> f32 {
        let size = self.thermometers.len() as f32;
        self.thermometers.iter()
            .fold(0.0, |sum, thermometer| sum + thermometer.get_temperature_c()) / size
    }

    pub fn get_thermometers(&self) -> &Vec<Thermometer> {
        &self.thermometers
    }

    pub fn read_temperatures(&self) {
        println!("Reading temperatures from {} thermometers...", self.thermometers.len());

        for thermometer in self.thermometers.iter() {
            thermometer.read_temperature().unwrap_or_else(|error| {
                println!("Failed to read temperature from thermometer {}: {}", thermometer.get_id(), error);
            });
        }
        println!("... finished reading");
    }

    fn search() -> Result<Vec<Thermometer>> {
        let files = match fs::read_dir(W1_PATH_PREFIX) {
            Ok(files) => files,
            Err(error) => {
                let env = std::env::var("ENV").unwrap_or("PROD".to_string());
                if env == "PROD" {
                    return Err(BrewtoothError::SystemError(format!("The one-wire driver is not active ({})", error)));

                } else {
                    return Ok(Vec::new())
                }
            }
        };

        let mut thermometers = Vec::new();
        for file in files {
            let filename = file?.file_name().into_string().unwrap();
            if filename.starts_with("28-") {
                thermometers.push(Thermometer::new(&filename));
            }
        }
        Ok(thermometers)
    }

}