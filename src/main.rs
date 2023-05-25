use crate::common::Configuration;
use crate::web::Server;

mod common;
mod web;

#[tokio::main]
async fn main() {

    let configuration = Configuration::read("config.yml").expect("Failed to read the config file");

    let server = Server::new();
    server.start(configuration).await;
}
