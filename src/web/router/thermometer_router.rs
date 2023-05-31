use axum::{
    routing::get,
    Router,
};

use crate::web::controllers::ThermometerController;

pub struct ThermometerRouter {
}

impl ThermometerRouter {
    pub fn router() -> Router {
        Router::new()
            .route("/", get(ThermometerController::get_thermometers))
    }
}
