use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::web::controllers::ThermometerController;

pub struct ThermometerRouter {
}

impl ThermometerRouter {
    pub fn router() -> Router {
        Router::new()
            .route("/", get(ThermometerController::get_thermometers))
            .route("/", post(ThermometerController::create_thermometer))
            .route("/:id", get(ThermometerController::get_thermometer))
            .route("/:id", put(ThermometerController::update_thermometer))
            .route("/:id", delete(ThermometerController::delete_thermometer))
    }
}
