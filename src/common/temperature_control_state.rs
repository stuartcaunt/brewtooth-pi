use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::common::TemperatureProfile;
use serde::Serialize;

#[derive(PartialEq, Eq, Clone, Serialize)]
pub enum ControlType {
    Setpoint,
    Profile,
}

impl fmt::Display for ControlType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ControlType::Setpoint => write!(f, "Setpoint"),
            ControlType::Profile => write!(f, "Profile"),
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureControlState {
    pub running: bool,
    pub current_time_s: u64,
    pub run_time_s: u64,
    pub temperature_c: f32,
    pub setpoint_c: f32,
    pub control_type: ControlType,
    pub temperature_profile: TemperatureProfile,
    pub controller_output: f32,
    pub heater_active: bool,
    pub agitator_active: bool,
    pub auto_temperature_control: bool,
    pub loop_ms: u64,
    pub sample_time_ms: u64,
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub output_max: f32,
    pub last_history_update_time_s: u64,
}

impl TemperatureControlState {
    pub fn defaults() -> TemperatureControlState {
        TemperatureControlState {
            running: false,
            current_time_s: 0,
            run_time_s: 0,
            temperature_c: 0.0,
            setpoint_c: 0.0,
            control_type: ControlType::Setpoint,
            temperature_profile: TemperatureProfile::new(),
            controller_output: 0.0,
            heater_active: false,
            agitator_active: false,
            auto_temperature_control: false,
            loop_ms: 0,
            sample_time_ms: 0,
            kp: 0.0,
            ki: 0.0,
            kd: 0.0,
            output_max: 0.0,
            last_history_update_time_s: 0,
        }
    }

    pub fn start_temperature_profile(&mut self) {
        if self.control_type == ControlType::Profile {
            let time_s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            self.setpoint_c = self.temperature_profile.start(time_s, self.temperature_c);
        }
    }

    pub fn stop_temperature_profile(&mut self) {
        if self.running && self.control_type == ControlType::Profile {
            self.temperature_profile.stop();
        }
    }

    pub fn update_temperature_profile(&mut self) {
        if self.running && self.control_type == ControlType::Profile {
            let time_s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            self.setpoint_c = self.temperature_profile.update(time_s, self.temperature_c);
        }
    }

    pub fn start_temperature_profile_pending_level(&mut self) {
        if self.running && self.control_type == ControlType::Profile {
            let time_s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

            self.temperature_profile.start_pending_level(time_s, self.temperature_c);
        }
    }

    pub fn terminate_temperature_profile_current_level(&mut self) {
        if self.running && self.control_type == ControlType::Profile {
            let time_s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

            self.temperature_profile.terminate_current_level(time_s, self.temperature_c);
        }
    }

}