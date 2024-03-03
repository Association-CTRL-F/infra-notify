use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Path to status file
    #[arg(short, long)]
    pub path: String,

    /// Recticprofile name
    #[arg(short, long)]
    pub name: String,
}
