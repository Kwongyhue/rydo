mod cli;
mod cmd;
mod error;

use clap::Parser;
use cli::Cli;
use cmd::{Command, TaskListManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Show { list_name } => {
            let tl_manager = TaskListManager::new(list_name.into());
            tl_manager.show_list()
        }
        Command::Create { list_name } => {
            let tl_manager = TaskListManager::new(list_name.into());
            tl_manager.create_list()
        }
        Command::Add {
            list_name,
            task_name,
        } => {
            let mut tl_manager = TaskListManager::new(list_name.into());
            tl_manager.add_task(task_name)
        }
        Command::SetActive {
            list_name,
            task_name,
        } => {
            let tl_manager = TaskListManager::new(list_name.into());
            tl_manager.set_active(task_name)
        }
        Command::SetInactive {
            list_name,
            task_name,
        } => {
            let tl_manager = TaskListManager::new(list_name.into());
            tl_manager.set_inactive(task_name)
        }
        Command::MarkComplete {
            list_name,
            task_name,
        } => {
            let tl_manager = TaskListManager::new(list_name.into());
            tl_manager.mark_complete(task_name)
        }
        Command::AddUrl {
            list_name,
            task_name,
            url,
        } => {
            let tl_manager = TaskListManager::new(list_name.into());
            tl_manager.add_url(task_name, url)
        }
    }?;

    Ok(())
}
