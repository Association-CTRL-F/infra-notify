use crate::parser::ResticProfiles;
use log::error;
use serde_json;
use std::{env::VarError, fmt::Debug, io, process};

pub struct Config {
    pub webhook_url: String,
    pub profile_name: String,
    pub profiles: ResticProfiles,
}

#[derive(thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration error: Missing environment variable WEBHOOK_URL")]
    WebhookUrlIsNotSet(#[from] VarError),
    #[error("Argument error: Profile does not exist in status file")]
    ProfileNotFound,
    #[error("IO error: Could not read status file")]
    CouldNotReadStatusFile(#[from] io::Error),
    #[error("Serde error: Could not deserialize status file")]
    CouldNotDeserializeStatusFile(#[from] serde_json::Error),
}

impl Debug for ConfigError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error!("{}", self.to_string());
        Ok(())
    }
}

pub trait Extract<T, E> {
    fn get_data_or_exit(self) -> T;
}

impl<T, E: std::fmt::Debug> Extract<T, E> for Result<T, E> {
    fn get_data_or_exit(self) -> T {
        match self {
            Ok(data) => data,
            Err(err) => {
                println!("{:?}", err);
                process::exit(1);
            }
        }
    }
}
