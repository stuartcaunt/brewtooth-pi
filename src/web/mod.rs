use std::sync::Arc;
use tokio::sync::Mutex;

use crate::common::Configuration;
use crate::services::ThermometerService;

pub use server::Server;

mod server;
mod router;
mod controllers;
mod dtos;
mod inputs;


#[derive(Clone)]
pub struct WebContext {
    configuration: Configuration,
    thermometer_service: Arc<Mutex<ThermometerService>>,
}

impl WebContext {
    pub fn new(configuration: Configuration, thermometer_service: Arc<Mutex<ThermometerService>>) -> Self {
        Self {
            configuration: configuration,
            thermometer_service: thermometer_service,
        }
    }
}
