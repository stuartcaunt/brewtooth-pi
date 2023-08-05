use std::sync::{Arc, Mutex};
use std::{
    fs,
    path::PathBuf,
};

use crate::common::*;

pub struct Thermometer {
    id: String,
    temperature: Arc<Mutex<f32>>,
}

impl Thermometer {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            temperature: Arc::new(Mutex::new(0.0))
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_temperature_c(&self) -> f32 {
        let temperature = self.temperature.lock().unwrap();

        *temperature
    }

    pub fn set_temperature_c(&self, value: f32) {
        let mut temperature = self.temperature.lock().unwrap();

        *temperature = value;
    }

    pub fn read_temperature(&self) -> Result<f32> {
        let mut path = PathBuf::from(W1_PATH_PREFIX);
        path.push(&self.id);
        path.push("temperature");
        let temperature_string = fs::read_to_string(path)?;
        let temperature = temperature_string.trim().parse::<u32>()?;

        let temperature = 0.001 * (temperature as f32);

        self.set_temperature_c(temperature);
        Ok(temperature)
    }
}