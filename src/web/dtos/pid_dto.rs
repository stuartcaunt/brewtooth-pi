use serde::{Serialize, Deserialize};

use crate::common::PIDParams;

#[derive(Serialize, Deserialize)]
pub struct PIDDto {
    kp: f32,
    ki: f32,
    kd: f32,
    output_max: f32,
}

impl From<PIDParams> for PIDDto {
    fn from(pid: PIDParams) -> Self {
        PIDDto {
            kp: pid.kp,
            ki: pid.ki,
            kd: pid.kd,
            output_max: pid.output_max,
        }
    }
}

impl From<PIDDto> for PIDParams {
    fn from(input: PIDDto) -> Self {
        PIDParams {
            kp: input.kp,
            ki: input.ki,
            kd: input.kd,
            output_max: input.output_max,
        }
    }
}