use axum::extract::Extension;
use std::sync::Arc;
use tokio::sync::Mutex;

//use crate::common::Context;

use crate::resources::*;

pub struct RootResource {

}

impl RootResource {

    pub async fn get_message(Extension(context): Extension<Arc<Mutex<Context>>>) -> String {
        let mut context = context.lock().await;

        let configuration = &mut context.configuration;

        let message = format!("Welcome to the Brewtooth-Pi server! We have {} temperature wires.", configuration.thermometers.len());

        configuration.clear_thermometers();

        message
    }

}