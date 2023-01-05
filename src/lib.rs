use reqwest::blocking::{Client, Response};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}

impl Message {
    pub fn send_message(&self, url: &str) -> Result<Response, Error> {
        let res = Client::new().post(url).json(&self).send()?;
        Ok(res)
    }
}
