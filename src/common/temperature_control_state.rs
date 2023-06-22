use crate::common::TemperatureProfile;

#[derive(PartialEq, Eq, Clone)]
pub enum ControlType {
    Setpoint,
    Profile,
}

#[derive(Clone)]
pub struct TemperatureControlState {
    pub running: bool,
    pub current_time_s: f32,
    pub run_time_s: f32,
    pub temperature_c: f32,
    pub setpoint_c: f32,
    pub control_type: ControlType,
    pub temperature_profile: TemperatureProfile,
    pub controller_output: f32,
    pub heater_active: bool,
    pub agitator_active: bool,
    pub auto_control: bool,
    pub loop_ms: u64,
    pub sample_time_ms: u64,
    pub window_size_ms: u32,
}

impl TemperatureControlState {
    pub fn new() -> TemperatureControlState {
        TemperatureControlState {
            running: false,
            current_time_s: 0.0,
            run_time_s: 0.0,
            temperature_c: 0.0,
            setpoint_c: 0.0,
            control_type: ControlType::Setpoint,
            temperature_profile: TemperatureProfile::new(),
            controller_output: 0.0,
            heater_active: false,
            agitator_active: false,
            auto_control: false,
            loop_ms: 0,
            sample_time_ms: 0,
            window_size_ms: 0,
        }
    }

    pub fn start(&mut self, time_s: f32, temperature_c: f32) {
        if self.control_type == ControlType::Profile {
            self.setpoint_c = self.temperature_profile.start(time_s, temperature_c);
        }
    }

    pub fn stop(&mut self, time_s: f32, temperature_c: f32) {
        if self.control_type == ControlType::Profile {
            self.temperature_profile.stop();
        }
    }

    pub fn update(&mut self, time_s: f32, temperature_c: f32) {
        if self.control_type == ControlType::Profile {
            self.setpoint_c = self.temperature_profile.update(time_s, temperature_c);
        }
    }

}