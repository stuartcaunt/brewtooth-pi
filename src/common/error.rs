use std::error::Error;
use std::result;
use std::fmt;

pub type Result<T> = result::Result<T, BrewtoothError>;

#[derive(Debug)]
pub enum BrewtoothError {
    IoError(std::io::Error),
    ConfigError(String),
}

impl Error for BrewtoothError {}

impl fmt::Display for BrewtoothError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrewtoothError::IoError(err) => writeln!(formatter, "IoError: {}", err),
            BrewtoothError::ConfigError(message) => writeln!(formatter, "ConfigError: {}", message),
        }
    }
}

impl From<std::io::Error> for BrewtoothError {
    fn from(err: std::io::Error) -> Self {
        BrewtoothError::IoError(err)
    }
}
