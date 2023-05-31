use axum::{
    routing::get,
    Router,
};

use crate::web::controllers::MashControllerController;

pub struct MashControllerRouter {
}

impl MashControllerRouter {
    pub fn router() -> Router {
        Router::new()
            .route("/", get(MashControllerController::get_mash_controller))
    }
}
