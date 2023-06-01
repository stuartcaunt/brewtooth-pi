use std::sync::{Arc, Mutex};
use std::{
    fs,
    path::PathBuf,
};

use crate::common::*;

pub struct Thermometer {
    id: String,
    temperature_mutex: Arc<Mutex<f32>>,
}

impl Thermometer {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            temperature_mutex: Arc::new(Mutex::new(0.0))
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_temperature_c(&self) -> f32 {
        let temperature_mutex = self.temperature_mutex.lock().unwrap();

        *temperature_mutex
    }

    pub fn set_temperature_c(&self, value: f32) {
        let mut temperature_mutex = self.temperature_mutex.lock().unwrap();

        *temperature_mutex = value;
    }

    pub fn read_temperature(&self) -> Result<()> {
        let mut path = PathBuf::from(W1_PATH_PREFIX);
        path.push(&self.id);
        path.push("temperature");
        let temperature_string = fs::read_to_string(path)?;
        let temperature = temperature_string.trim().parse::<u32>()?;

        self.set_temperature_c(0.001 * (temperature as f32));
        Ok(())
    }
}