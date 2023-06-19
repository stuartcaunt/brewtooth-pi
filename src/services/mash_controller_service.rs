use std::sync::Arc;
use crate::common::{MashController, MashControllerConfig, Result};
use crate::services::ThermometerService;

pub struct MashControllerService {
    mash_controller: MashController
}

impl MashControllerService {
    pub fn new(mash_controller_config: &MashControllerConfig,thermometer_service: &ThermometerService) -> Result<Self> {

        let thermometer_wire = Arc::clone(thermometer_service.get_thermometer_wire());
        Ok(Self {
            mash_controller: MashController::new(mash_controller_config, thermometer_wire)?
        })
    }

    pub fn get_mash_controller(&self) -> & MashController {
        &self.mash_controller
    }

}