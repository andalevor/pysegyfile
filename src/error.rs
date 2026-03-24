use std::array::TryFromSliceError;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyErr;
use segyfile::error;

pub enum WrpError {
    Segy(error::Error),
    Slice(TryFromSliceError),
}

impl From<error::Error> for WrpError {
    fn from(other: error::Error) -> Self {
        WrpError::Segy(other)
    }
}

impl From<TryFromSliceError> for WrpError {
    fn from(err: TryFromSliceError) -> Self {
        WrpError::Slice(err)
    }
}

impl From<WrpError> for PyErr {
    fn from(error: WrpError) -> Self {
        match error {
            WrpError::Segy(e) => PyValueError::new_err(e.to_string()),
            WrpError::Slice(e) => PyValueError::new_err(e.to_string()),
        }
    }
}

impl From<std::io::Error> for WrpError {
    fn from(err: std::io::Error) -> Self {
        WrpError::Segy(segyfile::error::Error::Io(err))
    }
}

impl From<std::str::Utf8Error> for WrpError {
    fn from(err: std::str::Utf8Error) -> Self {
        WrpError::Segy(segyfile::error::Error::Utf8(err))
    }
}
