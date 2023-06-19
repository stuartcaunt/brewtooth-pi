use std::{
    error::Error,
    result,
    fmt,
    num
};

pub type Result<T> = result::Result<T, BrewtoothError>;

#[derive(Debug)]
pub enum BrewtoothError {
    GpioError(rppal::gpio::Error),
    SystemError(String),
    IoError(std::io::Error),
    ConfigError(String),
    ParseError(num::ParseIntError),
}

impl Error for BrewtoothError {}

impl fmt::Display for BrewtoothError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrewtoothError::GpioError(error) => writeln!(formatter, "GpioError: {}", error),
            BrewtoothError::SystemError(message) => writeln!(formatter, "SystemError: {}", message),
            BrewtoothError::IoError(error) => writeln!(formatter, "IoError: {}", error),
            BrewtoothError::ConfigError(message) => writeln!(formatter, "ConfigError: {}", message),
            BrewtoothError::ParseError(message) => writeln!(formatter, "ParseError: {}", message),
        }
    }
}

impl From<std::io::Error> for BrewtoothError {
    fn from(error: std::io::Error) -> Self {
        BrewtoothError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for BrewtoothError {
    fn from(error: std::num::ParseIntError) -> Self {
        BrewtoothError::ParseError(error)
    }
}

impl From<rppal::gpio::Error> for BrewtoothError {
    fn from(error: rppal::gpio::Error) -> Self {
        BrewtoothError::GpioError(error)
    }
}
