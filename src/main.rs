use std::env;
use std::process::ExitCode;
use ureq::Error;
use chrono::Local;
use infra_notify::{send, Message};
use infra_notify::{DUMP_FAILURE, DUMP_SUCCESS, UPLOAD_FAILURE, UPLOAD_SUCCESS};
mod args;

fn main() -> ExitCode {
    let matches = args::cli().get_matches();
    let webhook_url = if let Ok(url) = env::var("WEBHOOK_URL") {
        url
    } else {
        println!("WEBHOOK_URL env var is not set");
        return ExitCode::FAILURE;
    };

    let mut msg: Message = match matches.subcommand() {
        Some(("dump-success", _)) => serde_json::from_str(DUMP_SUCCESS).expect("Will never fail"),
        Some(("dump-failure", _)) => serde_json::from_str(DUMP_FAILURE).expect("Will never fail"),
        Some(("upload-success", _)) => serde_json::from_str(UPLOAD_SUCCESS).expect("Will never fail"),
        Some(("upload-failure", _)) => serde_json::from_str(UPLOAD_FAILURE).expect("Will never fail"),
        _ => unreachable!(),
    };

    let timestamp = Local::now().to_rfc3339();
    msg.set_timestamp(timestamp);

    for _ in 0..4 {
        let res = send(&msg, &webhook_url);
        let res = match res {
            Err(Error::Transport(e)) => {
                println!("Warn: Transport error");
                println!("{:#?}", e);
                continue;
            }
            Err(Error::Status(code, response)) => {
                println!("Warn: HTTP Error: {} {}", code, response.status_text());
                continue;
            }
            Ok(res) => {
                break;
            }
        };
    }
    ExitCode::SUCCESS
}
