pub use server::Server;

mod server;
mod router;
mod controllers;

use crate::common::Configuration;

pub struct WebContext {
    configuration: Configuration,
}

impl WebContext {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration
        }
    }
}
