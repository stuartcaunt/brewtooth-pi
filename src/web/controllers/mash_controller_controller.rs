use axum::{
    extract::{Extension, Json, Path},
};

use crate::web::*;
use crate::common::{
    TemperatureControlState, 
    ControlType, 
    TemperatureProfile, 
    StateHistory
};
use crate::web::dtos::{
    MashControllerDto, 
    PIDDto, 
    ThermometerWireDto,
    RelayDto,
    TemperatureProfileInput,
};

pub struct MashControllerController {
}

impl MashControllerController {
    pub async fn get_mash_controller(Extension(context): Extension<WebContext>) -> Json<MashControllerDto> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        let mash_controller_dto = MashControllerDto::new(mash_controller);

        Json(mash_controller_dto)
    }

    pub async fn get_pids(Extension(context): Extension<WebContext>) -> Json<PIDDto> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();
        let pid = mash_controller.get_pid_params();

        Json(pid.into())
    }

    pub async fn set_pids(Extension(context): Extension<WebContext>, Json(input): Json<PIDDto>) -> Json<PIDDto> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();
        let pid = mash_controller.set_pid_params(&input.into());

        Json(pid.into())
    }

    pub async fn get_thermometer(Extension(context): Extension<WebContext>) -> Json<ThermometerWireDto> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();
        let thermometer_wire = mash_controller.get_thermometer_wire();

        let thermometer_wire_dto = ThermometerWireDto::new(thermometer_wire);

        Json(thermometer_wire_dto)
    }

    pub async fn get_temperature(Extension(context): Extension<WebContext>) -> String {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();
        let thermometer_wire = mash_controller.get_thermometer_wire();

        thermometer_wire.get_temperature().to_string()
    }

    pub async fn get_heater(Extension(context): Extension<WebContext>) -> Json<RelayDto> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();
        let heater = mash_controller.get_heater();

        let heater_dto = RelayDto::new(heater);

        Json(heater_dto)
    }

    pub async fn start_heater(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.set_heater_active(true);

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn stop_heater(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.set_heater_active(false);

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn get_agitator(Extension(context): Extension<WebContext>) -> Json<RelayDto> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();
        let agitator = mash_controller.get_agitator();

        let agitator_dto = RelayDto::new(agitator);

        Json(agitator_dto)
    }

    pub async fn start_agitator(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.set_agitator_active(true);

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn stop_agitator(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.set_agitator_active(false);

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn get_state(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn start_temperature_control_with_setpoint(Extension(context): Extension<WebContext>, Path(setpoint_c): Path<f32>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.set_setpoint_c(setpoint_c);
        mash_controller.start_temperature_control(ControlType::Setpoint);

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn start_temperature_control_with_profile(Extension(context): Extension<WebContext>, Json(temperature_profile_input): Json<TemperatureProfileInput>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.set_temperature_profile(temperature_profile_input.into());
        mash_controller.start_temperature_control(ControlType::Profile);

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn stop_temperature_control(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.stop_temperature_control();

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn set_temperature_control_automatic(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.set_auto_temperature_control(true);

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn set_temperature_control_manual(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.set_auto_temperature_control(false);

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn set_temperature_control_setpoint(Extension(context): Extension<WebContext>, Path(setpoint_c): Path<f32>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.set_setpoint_c(setpoint_c);

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn get_temperature_control_setpoint(Extension(context): Extension<WebContext>) -> String {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        let setpoint_c = mash_controller.get_setpoint_c();

        setpoint_c.to_string()
    }

    pub async fn get_temperature_profile(Extension(context): Extension<WebContext>) -> Json<TemperatureProfile> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        let profile = mash_controller.get_temperature_profile();

        Json(profile)
    }

    pub async fn start_temperature_profile_level(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.start_temperature_control_profile_level();

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn skip_temperature_profile_level(Extension(context): Extension<WebContext>) -> Json<TemperatureControlState> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        mash_controller.skip_temperature_control_profile_level();

        let state = mash_controller.get_temperature_control_state();

        Json(state)
    }

    pub async fn get_state_history(Extension(context): Extension<WebContext>) -> Json<Vec<StateHistory>> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        let state_history = mash_controller.get_state_history();

        Json(state_history)
    }

}