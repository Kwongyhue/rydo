use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Create{ list_name: String },
//     Add { task_name: String },
//     List { list_name: String },
//     SetActive { list_name: String, task_name: String, },
//     SetInactive {},
//     Complete {},
}

pub struct TaskList {
    pub path: PathBuf,
}

impl TaskList {
    pub fn new(path: PathBuf) -> Self {
        println!("new: {}", path.display());
        Self { path: path }
    }

    pub fn create_list(&self, list_name: String) {
        println!("i'm in {}", list_name);
    }
    // pub fn add_task() {}
    // pub fn mark_active() {}
    // pub fn mark_inactive() {}
    // pub fn mark_complete() {}
}

