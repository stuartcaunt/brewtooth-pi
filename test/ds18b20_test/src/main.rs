use std::error::Error;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use core::fmt::{Debug};
use one_wire_bus::OneWire;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

fn find_devices<P, E>(delay: &mut impl DelayUs<u16>, one_wire_pin: P) where P: OutputPin<Error=E> + InputPin<Error=E>, E: Debug {
    let mut one_wire_bus = OneWire::new(one_wire_pin).unwrap();
    for device_address in one_wire_bus.devices(false, delay) {
        // The search could fail at any time, so check each result. The iterator automatically
        // ends after an error.
        let device_address = device_address.unwrap();

        // The family code can be used to identify the type of device
        // If supported, another crate can be used to interact with that device at the given address
        println!("Found device at address {:?} with family code: {:#x?}", device_address, device_address.family_code());
    }
}


const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading temperature on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    let delayUs = 1000;

    find_devices(&mut delayUs, pin);

    Ok(())
}
