use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::{Result, BrewtoothError};

const MAX_POINTS_FOR_DERIVATIVE: usize = 4;

pub struct PIDController {
    last_time_ms: u64,
    last_inputs: Vec<f32>,
    last_output: f32,
    sample_time_ms: u32,
    output_min: f32,
    output_max: f32,
    i_term: f32,
    is_auto_mode: bool,
}


impl PIDController {
    pub fn new() -> PIDController {

        PIDController {
            last_time_ms: 0,
            last_inputs: Vec::new(),
            last_output: 0.0,
            sample_time_ms: 100,
            output_min: 0.0,
            output_max: 255.0,
            i_term: 0.0,
            is_auto_mode: false,
        }
    }

    pub fn set_sample_time_ms(&mut self, sample_time_ms: u32) {
        self.sample_time_ms = sample_time_ms;
    }

    pub fn set_output_limits(&mut self, output_min: f32, output_max: f32) {
        if output_min >= output_max {
            return;
        }

        self.output_min = output_min;
        self.output_max = output_max;

        if self.is_auto_mode {
            if self.last_output > output_max {
                self.last_output = output_max;
            
            } else if self.last_output < output_min {
                self.last_output = output_min
            }
            
            if self.i_term > output_max {
                self.i_term = output_max;
            
            } else if self.i_term < output_min {
                self.i_term = output_min
            }

        }
    }

    pub fn set_auto_mode(&mut self, is_auto_mode: bool) {
        if is_auto_mode != self.is_auto_mode {
            self.is_auto_mode = is_auto_mode;

            // initialise
            if is_auto_mode {
                self.i_term = 0.0;
                self.last_inputs.clear();
                let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
                self.last_time_ms = now_ms - self.sample_time_ms as u64;
            }
        } 
    }

    pub fn compute(&mut self, input: f32, setpoint: f32, kp: f32, ki: f32, kd: f32) -> Result<f32> {
        if !self.is_auto_mode {
            return Err(BrewtoothError::ControlError("PID Controller is manual".to_string()));
        }

        if kp < 0.0 || ki < 0.0 || kd < 0.0 {
            return Err(BrewtoothError::ControlError("PID values are invalid".to_string()));
        }

        let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let time_change_ms = (now_ms - self.last_time_ms) as u32;

        if time_change_ms < self.sample_time_ms {
            // println!("Insufficient time since last control call");
            return Ok(self.last_output);
        }

        let sample_time_s = (self.sample_time_ms / 1000) as f32;
   
        let ki = ki * sample_time_s;
        let kd = kd / sample_time_s;

        let error = setpoint - input;
        self.i_term += ki * error;

        if self.i_term > self.output_max {
            self.i_term = self.output_max;
        
        } else if self.i_term < self.output_min {
            self.i_term = self.output_min;
        }

        self.last_inputs.push(input);

        while self.last_inputs.len() > MAX_POINTS_FOR_DERIVATIVE {
            self.last_inputs.remove(0);
        }

        let d_input = self.calculate_derivative();
    
        let mut output = kp * error + self.i_term - kd * d_input;
        
        if output > self.output_max {
            output = self.output_max;
        
        } else if output < self.output_min {
            output = self.output_min;
        }

        self.last_time_ms = now_ms;
        self.last_output = output;

        Ok(output)
    }

    fn calculate_derivative(&self) -> f32 {
        let n_points = self.last_inputs.len();
        if n_points == 2 {
            return self.last_inputs[1] - self.last_inputs[0];
        
        } else if n_points == 3 {
            return 1.5 * self.last_inputs[2] - 2.0 * self.last_inputs[1] + 0.5 * self.last_inputs[0];
        
        } else if n_points == 4 {
            return 1.833333 * self.last_inputs[3] - 3.0 * self.last_inputs[2] + 1.5 * self.last_inputs[1] - 0.333333 * self.last_inputs[0];
        
        } else if n_points == 5 {
            return 2.083333 * self.last_inputs[4] - 4.0 * self.last_inputs[3] + 3.0 * self.last_inputs[2] - 1.333333 * self.last_inputs[1] + 0.25 * self.last_inputs[0];

        } else {
            return 0.0;
        }
    }

}