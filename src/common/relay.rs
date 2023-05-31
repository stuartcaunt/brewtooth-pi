use crate::common::*;

pub struct Relay {
    port: u8,
    is_active: bool,
}

impl Relay {
    pub fn new(port: u8) -> Result<Self> {
        // Verify port
        Ok(Self {
            port: port,
            is_active: false,
        })
    }

    pub fn get_port(&self) -> u8 {
        self.port
    }

    pub fn get_is_active(&self) -> bool {
        self.is_active
    }
}