use crate::args::Args;
use crate::embed::Message;
use chrono::Utc;
use clap::Parser;
use embed::EmbedBuilder;
use env_logger::Env;
use log::{error, info, warn};
use parser::ResticProfiles;
use std::{env, fs, process::ExitCode, thread, time};
use ureq::Error;
mod args;
mod embed;
mod generate_embed;
mod parser;

fn main() -> ExitCode {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let webhook_url = if let Ok(url) = env::var("WEBHOOK_URL") {
        url
    } else {
        error!("WEBHOOK_URL environmement var is not set");
        return ExitCode::FAILURE;
    };

    let status = match fs::read_to_string(&args.path) {
        Ok(status) => status,
        Err(err) => {
            error!("File read error: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let profiles = match serde_json::from_str::<ResticProfiles>(&status) {
        Ok(profiles) => profiles,
        Err(err) => {
            error!("Parser error: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let color = profiles.color(&args.name);
    let fields = profiles.generate_embed_fields(&args.name);
    let iso_timestamp = Utc::now().to_rfc3339();

    let r = EmbedBuilder::new()
        .title("VPS BACKUP STATUS")
        .color(color)
        .thumbnail("https://ctrl-f.io/assets/img/logo.png")
        .fields(fields)
        .timestamp(iso_timestamp)
        .build();

    let message = Message { embeds: vec![r] };

    for _ in 0..4 {
        let res = message.send(&webhook_url);
        match res {
            Err(boxed_err) => match *boxed_err {
                Error::Transport(e) => {
                    warn!("Transport error {}", e);
                    thread::sleep(time::Duration::from_millis(500));
                    continue;
                }
                Error::Status(code, response) => {
                    warn!("HTTP Error: {} {}", code, response.status_text());
                    thread::sleep(time::Duration::from_millis(500));
                    continue;
                }
            },
            Ok(_) => {
                info!("Notification successfully send");
                return ExitCode::SUCCESS;
            }
        };
    }
    error!("Could not send the notification");
    ExitCode::FAILURE
}
