use std::{
    fs,
    path::PathBuf
};

use crate::common::*;

pub struct Thermometer {
    id: String,
    temperature_c: f32,
}

impl Thermometer {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            temperature_c: 0.0
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_temperature_c(&self) -> f32 {
        self.temperature_c
    }

    pub fn read_temperature(&mut self) -> Result<()> {
        let mut path = PathBuf::from(W1_PATH_PREFIX);
        path.push(&self.id);
        path.push("temperature");
        let temperature_string = fs::read_to_string(path)?;
        let temperature = temperature_string.trim().parse::<u32>()?;

        self.temperature_c = 0.001 * (temperature as f32);
        Ok(())
    }
}