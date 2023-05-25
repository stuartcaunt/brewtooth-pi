pub use server::Server;

mod server;
mod router;
mod controllers;
mod dtos;

use crate::common::Configuration;
use crate::services::ThermometerService;

pub struct WebContext {
    configuration: Configuration,
    thermometer_service: ThermometerService,
}

impl WebContext {
    pub fn new(configuration: Configuration, thermometer_service: ThermometerService) -> Self {
        Self {
            configuration: configuration,
            thermometer_service: thermometer_service,
        }
    }
}
