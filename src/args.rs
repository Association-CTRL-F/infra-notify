use clap::{ArgGroup, Command};

pub fn cli() -> Command {
    Command::new("infra-notify")
        .subcommand_required(true)
        .subcommand(Command::new("dump-success"))
        .subcommand(Command::new("dump-failure"))
        .subcommand(Command::new("upload-success"))
        .subcommand(Command::new("upload-failure"))
        .group(ArgGroup::new("state"))
}
