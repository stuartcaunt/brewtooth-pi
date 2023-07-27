use std::sync::Arc;

use crate::common::Configuration;
use crate::services::{ThermometerService, MashControllerService};

pub use server::Server;

mod server;
mod router;
mod controllers;
mod dtos;


#[derive(Clone)]
pub struct WebContext {
    configuration: Configuration,
    thermometer_service: Arc<ThermometerService>,
    mash_controller_service: Arc<MashControllerService>,
}

impl WebContext {
    pub fn new(configuration: Configuration, thermometer_service: Arc<ThermometerService>, mash_controller_service: Arc<MashControllerService>) -> Self {
        Self {
            configuration: configuration,
            thermometer_service: thermometer_service,
            mash_controller_service: mash_controller_service,
        }
    }
}
