use one_wire_bus::{OneWire, OneWireError, Address, SearchState, OneWireResult};
use embedded_hal::blocking::delay::DelayMs;
use std::sync::{Mutex, Arc};
use ds18b20::{Ds18b20, SensorData};
use std::ops::DerefMut;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct TemperatureBus<P> where
    P: InputPin<Error>,
    P: OutputPin<Error>,
{
    bus: OneWire<P>,
    driver: Ds18b20
}

impl<P: InputPin<Error> + OutputPin<Error>> TemperatureBus<P> {
    pub fn new(pin: P) -> Result<Self, OneWireError<Error>> {

        match OneWire::new(pin) {
            Ok(mut onewire) => {

                let device = Self::find_device(&mut onewire).ok_or(OneWireError::Timeout)?;

                Ok(Self {
                    bus: onewire,
                    driver: Ds18b20::new::<EspError>(device).unwrap()
                })
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    fn find_device(bus: &mut OneWire<P>) -> Option<Address> {
        match bus.device_search(None, false, &mut delay::Ets) {
            Ok(Some((addr, _new_state))) => {
                println!("Found device at address: {}", addr.0);
                Some(addr)
            }
            Ok(None) => {
                println!("No device found!");
                None
            }
            Err(err) => {
                println!("Error while searching {:?}", err);
                None
            }
        }
    }

    fn start_measure(&mut self) -> OneWireResult<(), EspError> {
        self.driver.start_temp_measurement(&mut self.bus, &mut delay::Ets)
    }

    fn read_temperature(&mut self) -> OneWireResult<SensorData, EspError> {
        self.driver.read_data(&mut self.bus, &mut delay::Ets)
    }

    pub fn query_temperature(&mut self) -> OneWireResult<f32, EspError> {
        self.start_measure()?;
        // Wait for the measurement to be done
        delay::Ets.delay_ms(750u32);
        self.read_temperature().map(|data| data.temperature)
    }

}
