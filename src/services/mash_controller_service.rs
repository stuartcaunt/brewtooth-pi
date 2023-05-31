use crate::common::{MashController, MashControllerConfig, Result};

pub struct MashControllerService {
    mash_controller: MashController
}

impl MashControllerService {
    pub fn new(mash_controller_config: &MashControllerConfig) -> Result<Self> {
        Ok(Self {
            mash_controller: MashController::new(mash_controller_config)?
        })
    }

    pub fn get_mash_controller(&self) -> & MashController {
        &self.mash_controller
    }

}