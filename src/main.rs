use infra_notify::Message;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = env::var("WEBHOOK_URL").unwrap();
    let message = Message {
        content: "yo".to_string(),
    };

    let res = message.send_message(&webhook_url).unwrap();
    println!("Response status: {}", res.text().unwrap());
    Ok(())
}
