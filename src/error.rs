
use std::ops::Deref;
use std::ffi;
use std::error;
use std::fmt;
use std::str;
use std::string;
use cpufreq::CpuPowerError;

#[derive(Debug)]
pub enum Error {
    CpuPowerError(CpuPowerError),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::CpuPowerError(ref err) => write!(f, "CpuFreq error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::CpuPowerError(ref err) => error::Error::description(err)
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::CpuPowerError(ref err) => Some(err),
        }
    }
}

impl From<CpuPowerError> for Error {
    fn from(source: CpuPowerError) -> Error {
        Error::CpuPowerError(source)
    }
}
