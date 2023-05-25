
type DeviceAddress = [u8; 8];

pub struct Thermometer {
    address: DeviceAddress,
    temperature_c: f32,
}

impl Thermometer {
    pub fn get_address(&self) -> &DeviceAddress {
        &self.address
    }

    pub fn get_temerature_c(&self) -> f32 {
        self.temperature_c
    }
}