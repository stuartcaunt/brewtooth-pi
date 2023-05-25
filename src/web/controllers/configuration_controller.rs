use axum::extract::{Extension, Json};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::web::*;

pub struct ConfigurationController {
}

impl ConfigurationController {
    pub async fn get_configuration(Extension(context): Extension<Arc<Mutex<WebContext>>>) -> Json<Configuration> {
        let mut context = context.lock().await;

        let configuration = &mut context.configuration;

        Json(configuration.clone())
    }
}