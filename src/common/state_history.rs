use crate::common::TemperatureControlState;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateHistory {
    time_s: u64,
    temperature_c: f32,
    setpoint_c: f32,
    controller_output_percent: f32,
    heater_active: bool,
    agitator_active: bool,
}


impl StateHistory {
    pub fn new(state: &TemperatureControlState) -> StateHistory {
        StateHistory {
            time_s: state.current_time_s,
            temperature_c: state.temperature_c,
            setpoint_c: state.setpoint_c,
            controller_output_percent: 100.0 * (state.controller_output / state.output_max),
            heater_active: state.heater_active,
            agitator_active: state.agitator_active,
        }
    }
}