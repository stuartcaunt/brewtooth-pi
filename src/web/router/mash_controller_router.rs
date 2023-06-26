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
            .route("/pid", post(MashControllerController::get_pids))
    }
}
