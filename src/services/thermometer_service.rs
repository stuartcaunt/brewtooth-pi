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

    pub fn get_by_id_mut(&mut self, id: u32) -> Option<&mut ThermometerWire> {
        self.thermometer_wires.iter_mut()
            .find(|thermometer_wire| thermometer_wire.get_id() == id)
    }

    pub fn add(&mut self, thermometer_wire_config: &ThermometerWireConfig) -> Option<&ThermometerWire> {
        let mut thermometer_wire = ThermometerWire::new(thermometer_wire_config);
        let mut id = thermometer_wire.get_id();
        if id == 0 {
            let max_id = self.thermometer_wires.iter()
                .fold(0, |max, a_thermometer_wire| if a_thermometer_wire.get_id() > max { a_thermometer_wire.get_id() } else { max });

            id = max_id + 1;
            
            println!("Creating new thermometerWire with Id {}", id);
            thermometer_wire.set_id(id);

        } else {
            if let Some(existing_thermometer_index) = self.thermometer_wires.iter().position(|a_thermometer_wire| a_thermometer_wire.get_id() == id) {
                println!("Thermometer with Id {} already exists: replacing it", id);

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

    pub fn update(&mut self, id: u32, thermometer_wire_config: &ThermometerWireConfig) -> Option<&ThermometerWire> {
        if let Some(thermometer_wire) = self.get_by_id_mut(id) {

            // Release current port
            // GPIOService::_()->release(thermometerWire->getPort());

            thermometer_wire.set_name(&thermometer_wire_config.name);
            thermometer_wire.set_port(thermometer_wire_config.port);

            // Check/acquire GPIO port
            // thermometerWire->setPortIsValid(GPIOService::_()->acquire(thermometerWireConfig.port));
            // if (thermometerWire->getPortIsValid()) {
            //     GPIOService::_()->setPinMode(thermometerWire->getPort(), INPUT);
                
            //     thermometerWire->init();
            // } else {
            //     WARN("Thermometer \"%s\" has an invalid GPIO port", thermometerWire->getName().c_str(), thermometerWire->getPort());
            // }

            Some(thermometer_wire)

        } else {
            println!("Unable to update thermometerWire with Id {} as it does not exist", id);

            None
        }
    }

    pub fn delete(&mut self, id: u32) -> bool {
        if let Some(thermometer_wire) = self.get_by_id(id) {
            let existing_thermometer_index = self.thermometer_wires.iter().position(|a_thermometer_wire| a_thermometer_wire.get_id() == id).unwrap();


            // Release current port
            // GPIOService::_()->release(thermometerWire->getPort());

            self.thermometer_wires.remove(existing_thermometer_index);

            true

        } else {
            println!("Unable to delete thermometerWire with Id {} as it does not exist", id);

            false
        }
    }

    pub fn read_temperatures(&mut self) {
        println!("Reading temperatured from {} thermometers", self.thermometer_wires.len());
    }
}