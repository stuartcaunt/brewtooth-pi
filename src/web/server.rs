use crate::web::router::Router;
use crate::common::Configuration;

pub struct Server {
}

impl Server {

    pub fn new() -> Self {
        Self {
        }
    }

    pub async fn start(&self, configuration: Configuration) {
        let app = Router::router(configuration);

        // Run the server
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    }
}