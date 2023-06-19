use std::sync::Arc;
use crate::common::{ThermometerWire};
use crate::common::*;

pub struct ThermometerService {
    thermometer_wire: Arc<ThermometerWire>
}

impl ThermometerService {
    pub fn new() -> Result<Self> {
        let thermometer_wire = ThermometerWire::new()?;
        Ok(Self {
            thermometer_wire: Arc::new(thermometer_wire),
        })
    }

    pub fn get_thermometer_wire(&self) -> &Arc<ThermometerWire> {
        &self.thermometer_wire
    }

    pub fn read_temperatures(&self) {
        self.thermometer_wire.read_temperatures();
    }
}