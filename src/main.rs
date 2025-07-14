mod cli;
mod cmd;

use clap::Parser;
use cli::Cli;
use cmd::{Command, TaskList};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let task_list = TaskList::new("test.json".into());

    match cli.cmd {
        Command::Create { list_name } => task_list.create_list(list_name),
    }

    Ok(())
}
