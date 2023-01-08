use chrono::Local;
use infra_notify::{send, Message};
use infra_notify::{DUMP_FAILURE, DUMP_SUCCESS, UPLOAD_FAILURE, UPLOAD_SUCCESS};
use std::env;
use std::process::ExitCode;
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
        Some(("upload-success", _)) => {
            serde_json::from_str(UPLOAD_SUCCESS).expect("Will never fail")
        }
        Some(("upload-failure", _)) => {
            serde_json::from_str(UPLOAD_FAILURE).expect("Will never fail")
        }
        _ => unreachable!(),
    };

    let timestamp = Local::now().to_rfc3339();
    msg.set_timestamp(timestamp);

    println!("{:#?}", msg);
    let res = send(&msg, &webhook_url);
    let res = match res {
        Ok(r) => r,
        Err(e) => {
            println!("WARN: Network Error see bellow");
            println!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    if res.status().is_client_error() {
        println!(
            "WARN: Server returned an client error HTTP code: {}",
            res.status()
        );
        println!("{}", res.text().unwrap());
        ExitCode::FAILURE
    } else if res.status().is_server_error() {
        println!(
            "WARN: Server returned an server error HTTP code: {}",
            res.status()
        );
        ExitCode::FAILURE
    } else if res.status().is_success() {
        println!(
            "INFO: Successfully send the message HTTP code: {}",
            res.status()
        );
        ExitCode::SUCCESS
    } else {
        println!(
            "WARN: Server returned an unknown error HTTP code: {}",
            res.status()
        );
        ExitCode::FAILURE
    }
}
