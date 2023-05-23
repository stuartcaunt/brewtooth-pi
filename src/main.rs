use axum::{routing::get, Router, Extension};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::resources::RootResource;
use crate::resources::*;
use crate::common::Configuration;

mod resources;
mod common;

#[tokio::main]
async fn main() {

    let configuration = Configuration::read("config.yml").expect("Failed to read the config file");

    let app = Router::new()
        .route("/", get(RootResource::get_message))
        .layer(Extension(Arc::new(Mutex::new(Context::new(configuration)))));

    // Run the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
