use std::sync::{Arc, Mutex};

use crate::common::{
    Result, 
    MashControllerConfig,
    ThermometerWire,
    Relay,
    PIDParams,
    PIDConfig,
    TemperatureControlState,
    ControlType,
};

pub struct MashController {
    thermometer_wire: Arc<ThermometerWire>,
    heater: Relay,
    agitator: Relay,
    pid: Arc<Mutex<PIDParams>>,
    // temperature_controller: PID,
    state: Arc<Mutex<TemperatureControlState>>,
    last_time_ms: u64,
    start_time_ms: u64,
    window_start_time_ms: u64,
}

impl MashController {
    pub fn new(mash_controller_config: &MashControllerConfig, thermometer_wire: Arc<ThermometerWire>) -> Result<Self> {
        Ok(Self {
            thermometer_wire: thermometer_wire,
            heater: Relay::new(mash_controller_config.heater_port)?,
            agitator: Relay::new(mash_controller_config.agitator_port)?,
            pid: Arc::new(Mutex::new(PIDParams::new(&mash_controller_config.pid))),
            state: Arc::new(Mutex::new(TemperatureControlState::new())),
            last_time_ms: 0,
            start_time_ms: 0,
            window_start_time_ms: 0
        })
    }

    pub fn get_heater(&self) -> &Relay {
        &self.heater
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

    pub fn is_agitator_active(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.agitator_active
    }

    pub fn get_temperature(&self) -> f32 {
        self.thermometer_wire.get_temperature()
    }

    pub fn set_tunings(&self, kp: f32, ki: f32, kd: f32) {
        let mut pid = self.pid.lock().unwrap();

        pid.kp = kp;
        pid.ki = ki;
        pid.kd = kd;
    }

    pub fn get_kp(&self) -> f32 {
        let pid = self.pid.lock().unwrap();
        pid.kp
    }

    pub fn get_ki(&self) -> f32 {
        let pid = self.pid.lock().unwrap();
        pid.ki
    }

    pub fn get_kd(&self) -> f32 {
        let pid = self.pid.lock().unwrap();
        pid.kd
    }

    pub fn set_output_max(&self, output_max: u32) {
        let mut pid = self.pid.lock().unwrap();
        pid.output_max = output_max;
    }

    pub fn get_output_max(&self) -> u32 {
        let pid = self.pid.lock().unwrap();
        pid.output_max
    }

    pub fn get_pid_params(&self) -> PIDParams {
        let pid = self.pid.lock().unwrap();
        return *pid;
    }

    pub fn set_pid_params(&self, pid_params: &PIDParams) {
        let mut pid = self.pid.lock().unwrap();
        return *pid = pid_params.clone();
    }

    pub fn get_temperature_control_state(&self) -> TemperatureControlState {
        let state = self.state.lock().unwrap();
        *state
    }

    pub fn set_setpoint_c(&mut self, setpoint_c: f32) {
        let mut state = self.state.lock().unwrap();
        state.setpoint_c = setpoint_c;
    }

    pub fn get_setpoint_c(&self) -> f32 {
        let state = self.state.lock().unwrap();
        state.setpoint_c
    }


    // void setTemperatureProfile(TemperatureProfile temperatureProfile) {
    //     _state.temperatureProfile = temperatureProfile;
    // }
    
    // TemperatureProfile getTemperatureProfile() const {
    //     return _state.temperatureProfile;
    // }

    pub fn start_temperature_control(&self, control_type: ControlType) {
    
        // if (_temperatureController != NULL) {
        //     LOG("Temperature control is already active");
        //     return;
        // }
    
        // if (_heater != NULL) {
        //     // create temperature controller
        //     LOG("Starting temperature control with control type %s", toString(controlType).c_str());
        //     _state.setControlType(controlType);
        //     _state.controllerOutput = 0.0;
        //     _temperatureController = new PID(&_state.temperatureC, &_state.controllerOutput, &_state.setpointC, _config.pidParams.kp, _config.pidParams.ki, _config.pidParams.kd);
        //     _temperatureController->setOutputLimits(0.0, _config.pidParams.outputMax);
        //     this->setAutoTemperatureControl(true);
    
        //     _state.running = true;
        //     _startTimeMs = millis();
        //     _state.runTimeS = 0.0;
    
        //     // start the profile
        //     if (controlType == ControlType::Profile) {
        //         _state.setpointC= _state.temperatureProfile.start(0.001 * _startTimeMs, _state.temperatureC);
        //     }
    
        //     _temperatureController->setSampleTime(_state.sampleTimeMs);
    
        // } else {
        //     LOG("Cannot start temperature control since the heater is not configured");
        // }
    
    }
    
    pub fn stop_temperature_control() {

    }

    pub fn start_temperature_control_profile_level() {

    }

    pub fn skip_temperature_control_profile_level() {

    }

    pub fn update() {

    }
          
    // const String & getHistoryFileName() const {
    //     return _historyFileName;
    // }

}