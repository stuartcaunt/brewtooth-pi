use axum::extract::{Extension, Path};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::web::*;

pub struct ThermometerController {
}

impl ThermometerController {
    pub async fn get_thermometers(Extension(context): Extension<Arc<Mutex<WebContext>>>) -> String {
        let mut context = context.lock().await;

        let configuration = &mut context.configuration;

        let message = format!("We have {} temperature wires.", configuration.thermometers.len());

        configuration.clear_thermometers();

        message
    }

    pub async fn get_thermometer(Path(id): Path<u32>,Extension(context): Extension<Arc<Mutex<WebContext>>>) -> String {
        let mut context = context.lock().await;

        let configuration = &mut context.configuration;

        format!("thermometer {} of {} temperature wires.", id, configuration.thermometers.len())
    }
}