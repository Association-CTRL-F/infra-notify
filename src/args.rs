use clap::{crate_version, ArgGroup, Command};
pub fn cli() -> Command {
    Command::new("infra-notify")
        .version(crate_version!())
        .subcommand_required(true)
        .subcommand(Command::new("upload-success"))
        .subcommand(Command::new("upload-failure"))
        .group(ArgGroup::new("state"))
}
