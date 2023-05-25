use serde::Serialize;

use crate::common::ThermometerWire;

#[derive(Serialize)]
pub struct ThermometerDto {
    id: u32,
    port: u32,
    name: String,
    is_port_valid: bool,
    temperature: f32,
}

impl ThermometerDto {
    pub fn new(thermometer_wire: &ThermometerWire) -> Self {
        Self {
            id: thermometer_wire.get_id(),
            port: thermometer_wire.get_port(),
            name: thermometer_wire.get_name().to_string(),
            is_port_valid: thermometer_wire.is_port_valid(),
            temperature: thermometer_wire.get_temperature()
        }
    }
}