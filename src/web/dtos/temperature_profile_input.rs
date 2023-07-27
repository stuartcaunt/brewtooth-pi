use serde::{Serialize, Deserialize};

use crate::common::{TemperatureProfile, TemperatureLevel, ProfileState};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureLevelInput {
    pub name: String,
    pub setpoint_c: f32,
    pub duration_s: u64,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureProfileInput {
    levels: Vec<TemperatureLevelInput>,
}


impl From<TemperatureProfileInput> for TemperatureProfile {
    fn from(input: TemperatureProfileInput) -> Self {
        let mut temperature_profile = TemperatureProfile::new();

        temperature_profile.levels = input.levels.iter().map(|level_input| {
            let temperature_level = TemperatureLevel {
                name: level_input.name.clone(),
                setpoint_c: level_input.setpoint_c,
                duration_s: level_input.duration_s,
                tolerance_c: 0.0,
                timer_s: 0,
                start_time_s: 0,
                state: ProfileState::Inactive,
            };
            temperature_level
        }).collect();

        temperature_profile
    }
}