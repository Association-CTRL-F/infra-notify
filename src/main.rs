use chrono::Local;
use env_logger::Env;
use infra_notify::Message;
use infra_notify::{UPLOAD_FAILURE, UPLOAD_SUCCESS};
use log::{error, info, warn};
use std::process::ExitCode;
use std::{env, thread, time};
use ureq::Error;
mod args;

fn main() -> ExitCode {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let matches = args::cli().get_matches();
    let webhook_url = if let Ok(url) = env::var("WEBHOOK_URL") {
        url
    } else {
        error!("WEBHOOK_URL environmement var is not set");
        return ExitCode::FAILURE;
    };

    let mut msg: Message = match matches.subcommand() {
        Some(("upload-success", _)) => serde_json::from_str(UPLOAD_SUCCESS).unwrap(),
        Some(("upload-failure", _)) => serde_json::from_str(UPLOAD_FAILURE).unwrap(),
        _ => unreachable!(),
    };

    let timestamp = Local::now().to_rfc3339();
    msg.set_timestamp(&timestamp);

    for _ in 0..4 {
        let res = msg.send(&webhook_url);
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
