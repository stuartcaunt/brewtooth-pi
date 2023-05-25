use crate::common::{ThermometerWire, ThermometerWireConfig};

pub struct ThermometerService {
    thermometer_wires: Vec<ThermometerWire>
}

impl ThermometerService {
    pub fn new() -> Self {
        Self {
            thermometer_wires: Vec::new()
        }
    }

    pub fn get_all(&self) -> &Vec<ThermometerWire> {
        &self.thermometer_wires
    }

    pub fn get_by_id(&self, id: u32) -> Option<&ThermometerWire> {
        self.thermometer_wires.iter()
            .find(|&thermometer_wire| thermometer_wire.get_id() == id)
    }

    pub fn add(&mut self, thermometer_wire_config: & ThermometerWireConfig) -> Option<&ThermometerWire> {
        let mut thermometer_wire = ThermometerWire::new(thermometer_wire_config);
        let id = thermometer_wire.get_id();
        if id == 0 {
            let max_id = self.thermometer_wires.iter()
                .fold(0, |max, a_thermometer_wire| if a_thermometer_wire.get_id() > max { a_thermometer_wire.get_id() } else { max });

            let next_id = max_id + 1;
            // LOG("Creating new thermometerWire with Id %d", nextAvailableId);
            thermometer_wire.set_id(next_id);

        } else {

            if let Some(existing_thermometer_index) = self.thermometer_wires.iter().position(|a_thermometer_wire| a_thermometer_wire.get_id() == id) {
                // LOG("Thermometer with Id %d already exists: replacing it", thermometerWireConfig.id);
                self.thermometer_wires.remove(existing_thermometer_index);
            }
        }

        // Check if GPIO port is valid
        // thermometerWire->setPortIsValid(GPIOService::_()->acquire(thermometerWireConfig.port));
        // if (thermometerWire->getPortIsValid()) {
        //     GPIOService::_()->setPinMode(thermometerWire->getPort(), INPUT);
    
        //     thermometerWire->init();
    
        // } else {
        //     WARN("Thermometer \"%s\" has an invalid GPIO port", thermometerWire->getName().c_str(), thermometerWire->getPort());
        // }

        // Add the thermometer wire
        self.thermometer_wires.push(thermometer_wire);

        // Return the thermometer wire
        self.get_by_id(id)
    }

    pub fn read_temperatures(&mut self) {
        println!("Reading temperatured from {} thermometers", self.thermometer_wires.len());
    }
}