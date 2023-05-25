use axum::{
    routing::{get},
    Router,
};

use crate::web::controllers::ConfigurationController;

pub struct ConfigurationRouter {
}

impl ConfigurationRouter {
    pub fn router() -> Router {
        Router::new()
            .route("/", get(ConfigurationController::get_configuration))
    }
}
