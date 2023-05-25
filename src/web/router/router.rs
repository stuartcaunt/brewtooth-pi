use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    routing::{get},
    Extension,
    Router as AxumRouter,
};

use crate::common::Configuration;
use crate::web::controllers::RootController;
use crate::web::*;

use crate::web::router::ThermometerRouter;


const API_PREFIX: &str = "/api";

pub struct Router {
}

impl Router{

    pub fn router(configuration: Configuration) -> AxumRouter {

        AxumRouter::new()
            .nest(API_PREFIX, Router::api_router(configuration))
            .route("/", get(RootController::get_message))
    }
    
    fn api_router(configuration: Configuration) -> AxumRouter {
        
        AxumRouter::new()
            .nest("/thermometers", ThermometerRouter::router())
            .layer(Extension(Arc::new(Mutex::new(WebContext::new(configuration)))))
    }
}

