//! Error type for the client and matching Coinbase's API responses
use reqwest;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Structure to deserialize the details of Coinbase's API error responses
#[derive(Serialize, Deserialize, Debug)]
struct CbRequestErrorDetails {
    type_url: String,
    value: u8,
}

/// Structure to deserialize Coinbase's API error responses
#[derive(thiserror::Error, Serialize, Deserialize, Debug)]
pub struct CbRequestError {
    error: String,
    code: i32,
    message: String,
    details: CbRequestErrorDetails,
}

impl fmt::Display for CbRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

/// Enum accounting for the different error types arising from the client
#[derive(Debug, Error)]
pub enum CbError {
    #[error("http error {0}")]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("Coinbase: {0}")]
    Coinbase(CbRequestError),
}
