use serde::Deserialize;

use crate::common::ThermometerWireConfig;

#[derive(Deserialize)]
pub struct ThermometerInput {
    name: String,
    port: u32,
}

impl From<ThermometerInput> for ThermometerWireConfig {
    fn from(input: ThermometerInput) -> Self {
        ThermometerWireConfig {
            id: 0,
            name: input.name,
            port: input.port,
        }
    }
}