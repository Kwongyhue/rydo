mod cli;
mod cmd;

use clap::Parser;
use cli::Cli;
use cmd::{Command, TaskList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Show { list_name } => {
            let task_list = TaskList::new(list_name.into());
            task_list.show_list()
        }
        Command::Create { list_name } => {
            let task_list = TaskList::new(list_name.into());
            task_list.create_list()
        }
        Command::Add {
            list_name,
            task_name,
        } => {
            let mut task_list = TaskList::new(list_name.into());
            task_list.add_task(task_name)
        }
        Command::SetActive {
            list_name,
            task_name,
        } => {
            let task_list = TaskList::new(list_name.into());
            task_list.set_active(task_name)
        }
        Command::SetInactive {
            list_name,
            task_name,
        } => {
            let task_list = TaskList::new(list_name.into());
            task_list.set_inactive(task_name)
        }
        Command::MarkComplete {
            list_name,
            task_name,
        } => {
            let task_list = TaskList::new(list_name.into());
            task_list.mark_complete(task_name)
        }
    }?;

    Ok(())
}
