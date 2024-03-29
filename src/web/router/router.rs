use axum::{
    routing::{get},
    Extension,
    Router as AxumRouter,
};

use crate::web::controllers::RootController;
use crate::web::*;

use crate::web::router::{ConfigurationRouter, ThermometerRouter, MashControllerRouter};


const API_PREFIX: &str = "/api";

pub struct Router {
}

impl Router{

    pub fn router(web_context: WebContext) -> AxumRouter {

        AxumRouter::new()
            .nest(API_PREFIX, Router::api_router(web_context))
            .route("/", get(RootController::get_message))
    }
    
    fn api_router(web_context: WebContext) -> AxumRouter {

        AxumRouter::new()
            .nest("/thermometers", ThermometerRouter::router())
            .nest("/controller", MashControllerRouter::router())
            .nest("/configuration", ConfigurationRouter::router())
            .layer(Extension(web_context))
    }
}

