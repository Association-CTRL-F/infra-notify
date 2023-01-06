use args::Args;
use clap::Parser;
use infra_notify::{send, Message};
use std::env;
use std::process::ExitCode;
mod args;

fn main() -> ExitCode {
    let args = Args::parse();
    let webhook_url = if let Ok(url) = env::var("WEBHOOK_URL") {
        url
    } else {
        println!("WEBHOOK_URL env var is not set");
        return ExitCode::FAILURE;
    };

    let msg = Message {
        content: Some(args.message),
        username: Some(String::from("VPS Backup")),
        ..Default::default()
    };

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
        ExitCode::FAILURE
    } else if res.status().is_server_error() {
        println!(
            "WARN: Server returned an server error HTTP code: {}",
            res.status()
        );
        ExitCode::FAILURE
    } else if res.status().is_success() {
        println!("INFO: Successfully send the Message: {}", res.status());
        ExitCode::SUCCESS
    } else {
        println!(
            "WARN: Server returned an unknown error HTTP code: {}",
            res.status()
        );
        ExitCode::FAILURE
    }
}
