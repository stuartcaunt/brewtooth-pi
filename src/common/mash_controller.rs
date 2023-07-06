use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::{
    Result, 
    MashControllerConfig,
    ThermometerWire,
    Relay,
    PIDParams,
    ControlType,
    TemperatureControlState,
    TemperatureProfile,
    PIDController,
};

const WINDOW_SAMPLE_TIME_RATIO: f32 = 0.25;

struct ControllerTimer {
    last_time_ms: u64,
    start_time_ms: u64,
    window_start_time_ms: u64,
}

impl ControllerTimer {
    pub fn new() -> ControllerTimer {
        Self {
            last_time_ms: 0,
            start_time_ms: 0,
            window_start_time_ms: 0,
        }
    }
}

pub struct MashController {
    thermometer_wire: Arc<ThermometerWire>,
    heater: Relay,
    agitator: Relay,
    pid: Arc<Mutex<PIDParams>>,
    temperature_controller: Arc<Mutex<Option<PIDController>>>,
    state: Arc<Mutex<TemperatureControlState>>,
    timer: Arc<Mutex<ControllerTimer>>,
    window_size_ms: u64,
}

impl MashController {
    pub fn new(mash_controller_config: &MashControllerConfig, thermometer_wire: Arc<ThermometerWire>) -> Result<Self> {
        Ok(Self {
            thermometer_wire: thermometer_wire,
            heater: Relay::new(mash_controller_config.heater_port)?,
            agitator: Relay::new(mash_controller_config.agitator_port)?,
            pid: Arc::new(Mutex::new(PIDParams::new(&mash_controller_config.pid))),
            temperature_controller: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(TemperatureControlState {
                sample_time_ms: ((mash_controller_config.window_size_ms as f32) * WINDOW_SAMPLE_TIME_RATIO) as u32,
                ..TemperatureControlState::defaults()
            })),
            timer: Arc::new(Mutex::new(ControllerTimer::new())),
            window_size_ms: mash_controller_config.window_size_ms,
        })
    }

    pub fn get_heater(&self) -> &Relay {
        &self.heater
    }

    pub fn get_thermometer_wire(&self) -> &Arc<ThermometerWire> {
        &self.thermometer_wire
    }

    pub fn get_window_size_ms(&self) -> u64 {
        self.window_size_ms
    }

    pub fn set_heater_active(&self, active: bool) {
        if self.heater.is_active() != active {
            println!("Setting heater active {}", active);
            self.heater.set_active(active);

            let mut state = self.state.lock().unwrap();
            state.heater_active = true;
        }
    }

    pub fn is_heater_active(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.heater_active
    }

    pub fn get_agitator(&self) -> &Relay {
        &self.agitator
    }

    pub fn set_agitator_active(&self, active: bool) {
        if self.agitator.is_active() != active {
            println!("Setting agitator active {}", active);
            self.agitator.set_active(active);

            let mut state = self.state.lock().unwrap();
            state.agitator_active = true;
        }
    }

    pub fn get_temperature(&self) -> f32 {
        self.thermometer_wire.get_temperature()
    }

    pub fn get_pid_params(&self) -> PIDParams {
        let pid = self.pid.lock().unwrap();
        pid.clone()
    }

    pub fn set_pid_params(&self, pid_params: &PIDParams) -> PIDParams {
        let mut pid = self.pid.lock().unwrap();
        pid.kp = pid_params.kp;
        pid.ki = pid_params.ki;
        pid.kd = pid_params.kd;
        pid.output_max = pid_params.output_max;

        self.get_pid_params()
    }

    pub fn get_temperature_control_state(&self) -> TemperatureControlState {
        let state = self.state.lock().unwrap();
        state.clone()
    }

    pub fn set_setpoint_c(&self, setpoint_c: f32) {
        let mut state = self.state.lock().unwrap();
        state.setpoint_c = setpoint_c;
    }

    pub fn get_setpoint_c(&self) -> f32 {
        let state = self.state.lock().unwrap();
        state.setpoint_c
    }

    pub fn set_temperature_profile(&self, temperature_profile: TemperatureProfile) {
        let mut state = self.state.lock().unwrap();
        state.temperature_profile = temperature_profile;
    }

    pub fn get_temperature_profile(&self) -> TemperatureProfile {
        let state = self.state.lock().unwrap();
        state.temperature_profile.clone()
    }

    pub fn start_temperature_control(&self, control_type: ControlType) {

        let mut temperature_controller_option = self.temperature_controller.lock().unwrap();
        if !temperature_controller_option.is_none() {
            println!("Temperature control is already active");
            return;
        }

        // create temperature controller
        println!("Starting temperature control with control type {}", control_type.to_string());

        {
            let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
            let mut state = self.state.lock().unwrap();
            state.running = true;
            state.current_time_s = (now_ms / 1000) as f32;
            state.run_time_s = 0.0;
            state.temperature_c = self.get_temperature();
            state.control_type = control_type;
            state.controller_output = 0.0;
            // start the profile
            state.start_temperature_profile();

            let mut temperature_controller = PIDController::new();
            let pid = self.pid.lock().unwrap();
            temperature_controller.set_output_limits(0.0, pid.output_max);
            temperature_controller.set_sample_time_ms(state.sample_time_ms);
            *temperature_controller_option = Some(temperature_controller);

            let mut timer = self.timer.lock().unwrap();
            timer.start_time_ms = now_ms;
        }


        self.set_auto_temperature_control(true);
    }
    
    pub fn stop_temperature_control(&self) {
        {
            let mut temperature_controller_option = self.temperature_controller.lock().unwrap();
            // delete temperature controller
            if temperature_controller_option.is_some() {
                println!("Stopping/deleting temperature controller");
                *temperature_controller_option = None;
            }
        }

        {
            let mut state = self.state.lock().unwrap();

            state.controller_output = 0.0;
            state.running = false;
            state.run_time_s = 0.0;
    
            // stop the profile
            state.stop_temperature_profile();
        }

        self.set_heater_active(false);
        self.set_auto_temperature_control(false);
    }

    pub fn set_auto_temperature_control(&self, auto: bool) {
        let mut state = self.state.lock().unwrap();
        state.auto_temperature_control = auto;

        let temperature_controller_option = &mut *(self.temperature_controller.lock().unwrap());
        if let Some(temperature_controller) = temperature_controller_option {
            temperature_controller.set_auto_mode(auto);
        }
    }

    pub fn start_temperature_control_profile_level(&self) {
        let mut state = self.state.lock().unwrap();
        state.start_temperature_profile_pending_level();
    }

    pub fn skip_temperature_control_profile_level(&self) {
        let mut state = self.state.lock().unwrap();
        state.terminate_temperature_profile_current_level();
    }

    pub fn update(&self) {
        let mut state = self.state.lock().unwrap();
        let mut timer = self.timer.lock().unwrap();

        // Update loop timing
        let time_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;

        state.loop_ms = (time_ms - timer.last_time_ms) as u32;
        timer.last_time_ms = time_ms;

        state.current_time_s = (time_ms / 1000) as f32;

        // Get mean temperature from thermometer wires
        state.temperature_c = self.get_temperature();
        println!("Got average temperature of {}", state.temperature_c);

        // Shift relay window
        while (time_ms - timer.window_start_time_ms) > self.window_size_ms {
            timer.window_start_time_ms += self.window_size_ms;
        }
        
        // Update the profile
        state.update_temperature_profile();

        // Update runtime
        state.run_time_s = (time_ms - timer.start_time_ms) as f32 * 0.001;

        // Update temperate control
        let temperature_controller_option = &mut *(self.temperature_controller.lock().unwrap());
        if let Some(temperature_controller) = temperature_controller_option {

            let pid = self.pid.lock().unwrap();
            if let Ok(output) = temperature_controller.compute(state.temperature_c, state.setpoint_c, pid.kp, pid.ki, pid.kd) {
                state.controller_output = output;

                // Activate heater depending on controller output
                let window_factor = (time_ms - timer.window_start_time_ms) as f32 / self.window_size_ms as f32;
                let output_factor = state.controller_output as f32 / pid.output_max as f32;

                let active_heater = output_factor >= window_factor;

                if self.is_heater_active() != active_heater {
                    println!("{} : Changing heater state to {}, wf = {}, of = {}", state.run_time_s, active_heater, window_factor * 100.0, output_factor * 100.0);
                    self.set_heater_active(active_heater);
                }
            }
        }

        // Write data to file
        // this->writeHistoryToFile();
    }
          
    // const String & getHistoryFileName() const {
    //     return _historyFileName;
    // }

}