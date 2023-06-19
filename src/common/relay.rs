use std::sync::{Arc, Mutex};
use rppal::gpio::{Gpio, OutputPin};
use crate::common::Result;

pub struct Relay {
    port: u8,
    pin: Arc<Mutex<OutputPin>>,
}

impl Relay {
    pub fn new(port: u8) -> Result<Self> {
        // Verify port
        let pin = Relay::get_pin(port)?;

        Ok(Self {
            port: port,
            pin: Arc::new(Mutex::new(pin)),
        })
    }

    pub fn get_port(&self) -> u8 {
        self.port
    }

    pub fn is_active(&self) -> bool {
        let pin = self.pin.lock().unwrap();

        (*pin).is_set_high()
    }

    pub fn set_active(&self, activate: bool) {
        let mut pin = self.pin.lock().unwrap();

        if activate {
            (*pin).set_high();
        } else {
            (*pin).set_low();
        }
    }

    fn get_pin(port: u8) -> Result<OutputPin> {
        let pin = Gpio::new()?.get(port)?.into_output();

        Ok(pin)
    }
}