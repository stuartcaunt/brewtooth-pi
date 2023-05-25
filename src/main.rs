use std::{thread, time, sync::Arc};
use tokio::{task, sync::Mutex};

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

    let thermometer_service = Arc::new(Mutex::new(thermometer_service));
    let running = Arc::new(std::sync::Mutex::new(true));

    // Start thread to regularly update the thermometers
    let thermometer_service_clone = Arc::clone(&thermometer_service);
    let running_clone = Arc::clone(&running);
    let task = task::spawn(async move {
        // Loop while the web server is running
        while *running_clone.lock().unwrap() {
            thread::sleep(time::Duration::from_millis(1000));
            let mut thermometer_service = thermometer_service_clone.lock().await;
    
            thermometer_service.read_temperatures();
        }
    });

    let server = Server::new();
    server.start(configuration, thermometer_service).await;

    *running.lock().unwrap() = false;
    let _ = tokio::join!(task);
}
