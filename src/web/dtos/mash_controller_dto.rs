use serde::Serialize;

use crate::common::MashController;
use crate::web::dtos::RelayDto;

#[derive(Serialize)]
pub struct MashControllerDto {
    heater: RelayDto,
    agitator: RelayDto,
}

impl MashControllerDto {
    pub fn new(mash_controller: &MashController) -> Self {
        Self {
            heater: RelayDto::new(mash_controller.get_heater()),
            agitator: RelayDto::new(mash_controller.get_agitator())
        }
    }
}
