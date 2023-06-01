use crate::common::{ThermometerWire};
use crate::common::*;

pub struct ThermometerService {
    thermometer_wire: ThermometerWire
}

impl ThermometerService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            thermometer_wire: ThermometerWire::new()?
        })
    }

    pub fn get_thermometer_wire(&self) -> & ThermometerWire {
        &self.thermometer_wire
    }

    pub fn read_temperatures(&self) {
        self.thermometer_wire.read_temperatures();
    }
}