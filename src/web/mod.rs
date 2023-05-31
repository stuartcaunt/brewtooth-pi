use std::sync::Arc;
use tokio::sync::Mutex;

use crate::common::Configuration;
use crate::services::{ThermometerService, MashControllerService};

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
    mash_controller_service: Arc<Mutex<MashControllerService>>,
}

impl WebContext {
    pub fn new(configuration: Configuration, thermometer_service: Arc<Mutex<ThermometerService>>, mash_controller_service: Arc<Mutex<MashControllerService>>) -> Self {
        Self {
            configuration: configuration,
            thermometer_service: thermometer_service,
            mash_controller_service: mash_controller_service,
        }
    }
}
