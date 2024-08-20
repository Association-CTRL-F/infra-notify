use crate::args::Args;
use crate::config::Extract;
use crate::embed::Message;
use chrono::Utc;
use clap::Parser;
use config::Config;
use config::ConfigError;
use embed::EmbedBuilder;
use env_logger::Env;
use log::{error, info, warn};
use parser::ResticProfiles;
use std::{env, fs, process::ExitCode, thread, time};
use ureq::Error;
mod args;
mod config;
mod embed;
mod generate_embed;
mod parser;

fn main() -> ExitCode {
    let config = setup().get_data_or_exit();
    let fields = config
        .profiles
        .clone()
        .generate_embed_fields(&config.profile_name)
        .get_data_or_exit();

    let color = config.profiles.color(&config.profile_name);
    let iso_timestamp = Utc::now().to_rfc3339();

    let r = EmbedBuilder::new()
        .title("VPS BACKUP STATUS")
        .color(color)
        .thumbnail("https://ctrl-f.info/assets/img/logo.png")
        .fields(fields)
        .timestamp(iso_timestamp)
        .build();

    let message = Message { embeds: vec![r] };

    for _ in 0..4 {
        let res = message.send(&config.webhook_url);
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

fn setup() -> Result<Config, ConfigError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let profile_name = args.name;
    let webhook_url = env::var("WEBHOOK_URL")?;
    let profiles = fs::read_to_string(&args.path)?;
    let profiles = serde_json::from_str::<ResticProfiles>(&profiles)?;

    Ok(Config {
        webhook_url,
        profile_name,
        profiles,
    })
}
