use crate::common::*;
use crate::common::MashControllerConfig;

pub struct MashController {
    heater: Relay,
    agitator: Relay,
}

impl MashController {
    pub fn new(mash_controller_config: &MashControllerConfig) -> Result<Self> {
        Ok(Self {
            heater: Relay::new(mash_controller_config.heater_port)?,
            agitator: Relay::new(mash_controller_config.agitator_port)?,
        })
    }

    pub fn get_heater(&self) -> &Relay {
        &self.heater
    }

    pub fn get_agitator(&self) -> &Relay {
        &self.agitator
    }
}