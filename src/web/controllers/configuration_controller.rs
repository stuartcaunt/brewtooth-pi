use axum::extract::{Extension, Json};

use crate::web::*;

pub struct ConfigurationController {
}

impl ConfigurationController {
    pub async fn get_configuration(Extension(context): Extension<WebContext>) -> Json<Configuration> {
        let configuration = &context.configuration;

        Json(configuration.clone())
    }
}