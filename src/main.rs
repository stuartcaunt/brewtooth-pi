use std::{thread, time, sync::Arc};
use tokio::task;

use crate::common::Configuration;
use crate::web::Server;
use crate::services::{ThermometerService, MashControllerService};

mod common;
mod web;
mod services;

#[tokio::main]
async fn main() {

    // Read the configuration file
    let configuration = Configuration::read("config.yml").expect("Failed to read the config file");

    // Create the thermometer service and initialise thermometer wires
    let thermometer_service = ThermometerService::new().unwrap_or_else(|error| {
        println!("Failed to create thermometer service: {}", error);
        std::process::exit(1);
    });

    // Create the mash controller service and validate heater and agitator ports
    let mash_controller_service = MashControllerService::new(&configuration.mash_controller, &thermometer_service).unwrap_or_else(|error| {
        println!("Failed to create mash controller service: {}", error);
        std::process::exit(1);
    });

    let thermometer_service = Arc::new(thermometer_service);
    let mash_controller_service = Arc::new(mash_controller_service);
    let running = Arc::new(std::sync::Mutex::new(true));

    // Start thread to regularly update the thermometers
    let thermometer_service_clone = Arc::clone(&thermometer_service);
    let mash_controller_service_clone = Arc::clone(&mash_controller_service);
    let temperature_running_clone = Arc::clone(&running);
    let controller_running_clone = Arc::clone(&running);
    let temperature_read_task = task::spawn(async move {
        // Loop while the web server is running
        while *temperature_running_clone.lock().unwrap() {
            thermometer_service_clone.read_temperatures();

            thread::sleep(time::Duration::from_millis(5000));
        }
    });

    let controller_update_task = task::spawn(async move {
        // Loop while the web server is running
        while *controller_running_clone.lock().unwrap() {
            mash_controller_service_clone.update_mash_controller();

            thread::sleep(time::Duration::from_millis(500));
        }
    });

    let server = Server::new();
    server.start(configuration, thermometer_service, mash_controller_service).await;

    *running.lock().unwrap() = false;
    let _ = tokio::join!(temperature_read_task);
    let _ = tokio::join!(controller_update_task);
}
