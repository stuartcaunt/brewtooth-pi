pub use root_resource::RootResource;

mod root_resource;

use crate::common::Configuration;

pub struct Context {
    configuration: Configuration,
}

impl Context {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration
        }
    }
}