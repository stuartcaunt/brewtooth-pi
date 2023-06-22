use crate::common::PIDConfig;

#[derive(Clone)]
pub struct PIDParams {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub output_max: u32,
}

impl PIDParams {
    pub fn new(pid: &PIDConfig) -> PIDParams {
        PIDParams {
            kp: pid.kp,
            ki: pid.ki,
            kd: pid.kd,
            output_max: pid.output_max,
        }
    }
}