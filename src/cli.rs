use crate::cmd::Command;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rydo")]
#[command(about = "", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}
