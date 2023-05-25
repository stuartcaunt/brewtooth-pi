use crate::common::{Thermometer, ThermometerWireConfig};

pub struct ThermometerWire {

    id: u32,
    port: u32,
    name: String,
    is_port_valid: bool,
    
    is_temperature_reading: bool,

    thermometers: Vec<Thermometer>
    
}

impl ThermometerWire {

    pub fn new(config: &ThermometerWireConfig) -> Self {
        Self {
            id: config.id,
            port: config.port,
            name: config.name.clone(),
            is_port_valid: config.is_port_valid,
            is_temperature_reading: false,
            thermometers: Vec::new()
        }
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn get_port(&self) -> u32 {
        return self.port;
    }

    pub fn set_port(&mut self, port: u32) {
        self.port = port;
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn is_port_valid(&self) -> bool {
        return self.is_port_valid;
    }

    pub fn set_is_port_valid(&mut self, is_port_valid: bool) {
        self.is_port_valid = is_port_valid;
    }

    pub fn is_temperature_reading(&self) -> bool {
        return self.is_temperature_reading;
    }

    pub fn set_is_temperature_reading(&mut self, is_temperature_reading: bool) {
        self.is_temperature_reading = is_temperature_reading;
    }

    pub fn get_temperature(&self) -> f32 {
        let size = self.thermometers.len() as f32;
        self.thermometers.iter()
            .fold(0.0, |sum, thermometer| sum + thermometer.get_temerature_c()) / size
    }

}