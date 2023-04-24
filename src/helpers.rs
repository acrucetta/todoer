use std::io;
use thiserror::Error;

pub fn handle_error(message: &str) {
    eprintln!("Error: {}", message);
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to find configuration directory")]
    ConfigDirNotFound,

    #[error("{0}")]
    IOError(String, #[source] io::Error),

    #[error("Failed to get user input")]
    UserInputError(#[from] io::Error),

    #[error("Failed to read the stored Notion keys")]
    ConfigReadError,

    #[error("Failed to parse the stored Notion keys")]
    ConfigParseError,

    #[error("{0}")]
    JsonError(String, #[source] serde_json::Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),

    #[error("{0}")]
    ReqwestError(String, #[source] reqwest::Error),
}
