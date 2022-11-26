//! Midtransclient Custom Error

use std::collections::HashMap;
use std::fmt;
use std::error::{self, Error};
use std::num::ParseIntError;
use serde_json::Value;

/// Midtransclient API Error Struct
#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub status_code: u16,
    pub response: HashMap<String, Value>
}

impl error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ApiError {
    pub fn new(status_code: u16, response: HashMap<String, Value>, message: String) -> Self {
        Self { status_code, response, message }
    }
}

/// Midtransclient Custom Errors
pub enum MidtransError {
    RequestError(reqwest::Error),
    ParseError(ParseIntError),
    JsonDecodeError(serde_json::Error),
    ApiError(ApiError)
}

impl error::Error for MidtransError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MidtransError::RequestError(ref e) => Some(e),
            MidtransError::ParseError(ref e) => Some(e),
            MidtransError::JsonDecodeError(ref e) => Some(e),
            MidtransError::ApiError(ref e) => Some(e)
        }
    }
}

impl fmt::Display for MidtransError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MidtransError::RequestError(_) => write!(f, "Request Error"),
            MidtransError::ParseError(_) => write!(f, "Parse Int Error"),
            MidtransError::JsonDecodeError(_) => write!(f, "Fail to decode JSON string"),
            MidtransError::ApiError(_) => write!(f, "Midtrans API Error")
        }
    }
}

impl fmt::Debug for MidtransError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n", self)?;
        let mut current = self.source();
        while let Some(cause) = current {
            writeln!(f, "Caused by: \n\t{}", cause)?;
            current = cause.source();
        }
        Ok(())
    }
}

impl From<reqwest::Error> for MidtransError {
    fn from(err: reqwest::Error) -> Self {
        MidtransError::RequestError(err)
    }
}

impl From<serde_json::Error> for MidtransError {
    fn from(err: serde_json::Error) -> Self {
        MidtransError::JsonDecodeError(err)
    }
}

impl From<ParseIntError> for MidtransError {
    fn from(err: ParseIntError) -> Self {
        MidtransError::ParseError(err)
    }
}

impl From<ApiError> for MidtransError {
    fn from(err: ApiError) -> Self {
        MidtransError::ApiError(err)
    }
}
