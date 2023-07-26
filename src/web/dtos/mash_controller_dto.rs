use serde::Serialize;

use crate::common::MashController;
use crate::web::dtos::{RelayDto, PIDDto, ThermometerWireDto};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MashControllerDto {
    heater: RelayDto,
    agitator: RelayDto,
    pids: PIDDto,
    thermometers: ThermometerWireDto,
    window_size_ms: u64,
}

impl MashControllerDto {
    pub fn new(mash_controller: &MashController) -> Self {
        Self {
            heater: RelayDto::new(mash_controller.get_heater()),
            agitator: RelayDto::new(mash_controller.get_agitator()),
            pids: mash_controller.get_pid_params().into(),
            thermometers: ThermometerWireDto::new(mash_controller.get_thermometer_wire()),
            window_size_ms: mash_controller.get_window_size_ms(),
        }
    }
}
