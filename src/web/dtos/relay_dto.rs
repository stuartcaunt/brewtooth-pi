use serde::Serialize;

use crate::common::Relay;

#[derive(Serialize)]
pub struct RelayDto {
    port: u8,
    is_active: bool,
}

impl RelayDto {
    pub fn new(relay: &Relay) -> Self {
        Self {
            port: relay.get_port(),
            is_active: relay.is_active(),
        }
    }
}
