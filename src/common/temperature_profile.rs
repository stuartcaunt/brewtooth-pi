use serde::{Serialize, Deserialize};
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ProfileState {
    Inactive,
    Pending,
    Active,
    Terminated
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TemperatureLevel {
    pub name: String,
    pub setpoint_c: f32,
    pub tolerance_c: f32,
    pub duration_s: f32,
    pub timer_s: f32,
    pub start_time_s: f32,
    pub state: ProfileState,
}


impl TemperatureLevel {

    pub fn init(&mut self, tolerance_c: f32) {
        self.tolerance_c = tolerance_c;
        self.timer_s = 0.0;
        self.start_time_s = 0.0;
        self.state = ProfileState::Inactive;
    }

    pub fn start(&mut self, time_s: f32, temperature_c: f32) {
        if self.state == ProfileState::Inactive {
            if temperature_c < (self.setpoint_c - self.tolerance_c) {
                self.state = ProfileState::Pending;

            } else {
                self.start_time_s = time_s;
                self.state = ProfileState::Active;
            }
        }
    }

    pub fn stop(&mut self) {
        self.state = ProfileState::Inactive;
    }

    pub fn update(&mut self, time_s: f32, temperature_c: f32) {
        if self.state == ProfileState::Pending {
            if temperature_c >= (self.setpoint_c - self.tolerance_c) {
                self.state = ProfileState::Active;
                self.start_time_s = time_s;
            }

        } else if self.state == ProfileState::Active {
            self.timer_s = time_s - self.start_time_s;
            if self.timer_s >= self.duration_s {
                self.state = ProfileState::Terminated;
            }
        }
    }

    pub fn force_active_if_pending(&mut self, time_s: f32) {
        if self.state == ProfileState::Pending {
            self.state = ProfileState::Active;
            self.start_time_s = time_s;
        }
    }

    pub fn force_terminated(&mut self) {
        self.state = ProfileState::Terminated;
    }

}

#[derive(Clone, Serialize, Deserialize)]
pub struct TemperatureProfile {
    pub start_time_s: f32,
    pub tolerance_c: f32,
    pub state: ProfileState,
    pub levels: Vec<TemperatureLevel>,
    pub active_level: i32,
}

impl TemperatureProfile {
    pub fn new() -> TemperatureProfile {
        TemperatureProfile {  
            start_time_s: 0.0,
            tolerance_c: 0.0,
            state: ProfileState::Inactive,
            levels: Vec::new(),
            active_level: -1
        }
    }

    pub fn start(&mut self, time_s: f32, temperature_c: f32) -> f32 {
        if self.state != ProfileState::Inactive {
            if self.active_level >=0 {
                let level = &self.levels[self.active_level as usize];

                return level.setpoint_c;
            }

            return 0.0;
        }

        // Initialise levels
        for level in self.levels.iter_mut() {
            level.init(self.tolerance_c);
        }
        
        // Start first level
        self.state = ProfileState::Active;
        self.start_time_s = time_s;

        let level = &mut self.levels[0];
        self.active_level = 0;
        level.start(time_s, temperature_c);
        level.setpoint_c
    }

    pub fn stop(&mut self) {
        // Terminate all levels
        for level in self.levels.iter_mut() {
            level.stop();
        }

        // Reset active level and state
        self.active_level = -1;
        self.state = ProfileState::Inactive;
    }

    pub fn start_pending_level(&mut self, time_s: f32, temperature_c: f32) {
        if self.active_level >= 0 && self.state != ProfileState::Terminated {
            let level = &mut self.levels[self.active_level as usize];

            level.force_active_if_pending(time_s);
            self.update(time_s, temperature_c);
        }
    }

    pub fn terminate_current_level(&mut self, time_s: f32, temperature_c: f32) {
        if self.active_level >= 0 && self.state != ProfileState::Terminated {
            let level = &mut self.levels[self.active_level as usize];

            level.force_terminated();
            self.update(time_s, temperature_c);
        }
    }


    pub fn update(&mut self, time_s: f32, temperature_c: f32) -> f32 {
        if self.active_level >= 0 && self.state != ProfileState::Terminated {

            let number_of_levels = self.levels.len() as i32;

            let mut level = &mut self.levels[self.active_level as usize];

            level.update(time_s, temperature_c);

            // Go to next state
            if level.state == ProfileState::Terminated {
                self.active_level += 1;

                if self.active_level >= number_of_levels {
                    self.state = ProfileState::Terminated;
                    self.active_level = (number_of_levels - 1) as i32;

                } else {
                    level = &mut self.levels[self.active_level as usize];
                    level.start(time_s, temperature_c);
                }
            }

            return level.setpoint_c;

        } else if self.active_level >= 0 && self.state == ProfileState::Terminated {
            // Return setpoint of last active level
            let level = &mut self.levels[self.active_level as usize];

            return level.setpoint_c;
        }
        
        0.0
    }

}