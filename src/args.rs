use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[arg(short = 'm', long)]
    pub message: String,
}
