use serde::Serialize;

use crate::common::{ThermometerWire, Thermometer};

#[derive(Serialize)]
pub struct ThermometerDto {
    id: String,
    temperature: f32,
}

#[derive(Serialize)]
pub struct ThermometerWireDto {
    temperature: f32,
    thermometers: Vec<ThermometerDto>
}

impl ThermometerDto {
    pub fn new(thermometer: &Thermometer) -> Self {
        Self {
            id: thermometer.get_id().to_string(),
            temperature: thermometer.get_temperature_c(),
        }
    }
}

impl ThermometerWireDto {
    pub fn new(thermometer_wire: &ThermometerWire) -> Self {
        let thermometers = thermometer_wire.get_thermometers().iter().map(|thermometer| ThermometerDto::new(thermometer)).collect();
        Self {
            temperature: thermometer_wire.get_temperature(),
            thermometers: thermometers
        }
    }
}