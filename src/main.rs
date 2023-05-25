use crate::common::Configuration;
use crate::web::Server;
use crate::services::ThermometerService;

mod common;
mod web;
mod services;

#[tokio::main]
async fn main() {

    // Read the configuration file
    let configuration = Configuration::read("config.yml").expect("Failed to read the config file");

    // Create the thermometer service and initialise thermometer wires
    let mut thermometer_service = ThermometerService::new();
    for thermometer_wire_config in configuration.thermometers.iter() {
        thermometer_service.add(thermometer_wire_config);
    }

    let server = Server::new();
    server.start(configuration, thermometer_service).await;
}
