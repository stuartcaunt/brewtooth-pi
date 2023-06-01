use std::sync::Arc;

use crate::web::router::Router;
use crate::common::Configuration;
use crate::web::WebContext;
use crate::services::{ThermometerService, MashControllerService};

pub struct Server {
}

impl Server {

    pub fn new() -> Self {
        Self {
        }
    }

    pub async fn start(&self, configuration: Configuration, thermometer_service: Arc<ThermometerService>, mash_controller_service: Arc<MashControllerService>) {

        let web_context = WebContext::new(configuration, thermometer_service, mash_controller_service);

        let app = Router::router(web_context);

        // Run the server
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    }
}