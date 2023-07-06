use axum::{
    routing::{get, post},
    Router,
};

use crate::web::controllers::MashControllerController;

pub struct MashControllerRouter {
}

impl MashControllerRouter {
    pub fn router() -> Router {
        Router::new()
            .route("/", get(MashControllerController::get_mash_controller))
            .route("/pid", get(MashControllerController::get_pids))
            .route("/pid", post(MashControllerController::set_pids))
            .route("/thermometer", get(MashControllerController::get_thermometer))
            .route("/temperature", get(MashControllerController::get_temperature))
            .route("/heater", get(MashControllerController::get_heater))
            .route("/heater/start", post(MashControllerController::start_heater))
            .route("/heater/stop", post(MashControllerController::stop_heater))
            .route("/agitator", get(MashControllerController::get_agitator))
            .route("/agitator/start", post(MashControllerController::start_agitator))
            .route("/agitator/stop", post(MashControllerController::stop_agitator))
            .route("/state", get(MashControllerController::get_state))
            .route("/start/:setpoint_c", post(MashControllerController::start_temperature_control_with_setpoint))
            .route("/start", post(MashControllerController::start_temperature_control_with_profile))
            .route("/stop", post(MashControllerController::stop_temperature_control))
            .route("/automatic", post(MashControllerController::set_temperature_control_automatic))
            .route("/manual", post(MashControllerController::set_temperature_control_manual))
            .route("/setpoint/:setpoint_c", post(MashControllerController::set_temperature_control_setpoint))
            .route("/setpoint", get(MashControllerController::get_temperature_control_setpoint))
            .route("/profile", get(MashControllerController::get_temperature_profile))
            .route("/profile/start", get(MashControllerController::start_temperature_profile_level))
            .route("/profile/skip", get(MashControllerController::skip_temperature_profile_level))
    }
}
