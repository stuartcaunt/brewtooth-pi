use std::{
    error::Error,
    result,
    fmt,
    num
};

pub type Result<T> = result::Result<T, BrewtoothError>;

#[derive(Debug)]
pub enum BrewtoothError {
    SystemError(String),
    IoError(std::io::Error),
    ConfigError(String),
    ParseError(num::ParseIntError),
}

impl Error for BrewtoothError {}

impl fmt::Display for BrewtoothError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrewtoothError::SystemError(message) => writeln!(formatter, "SystemError: {}", message),
            BrewtoothError::IoError(err) => writeln!(formatter, "IoError: {}", err),
            BrewtoothError::ConfigError(message) => writeln!(formatter, "ConfigError: {}", message),
            BrewtoothError::ParseError(message) => writeln!(formatter, "ParseError: {}", message),
        }
    }
}

impl From<std::io::Error> for BrewtoothError {
    fn from(err: std::io::Error) -> Self {
        BrewtoothError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for BrewtoothError {
    fn from(err: std::num::ParseIntError) -> Self {
        BrewtoothError::ParseError(err)
    }
}
