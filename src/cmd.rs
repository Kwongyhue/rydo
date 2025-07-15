mod tasks;

use chrono::{DateTime, Local};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use tasks::Task;

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Create { list_name: String },
    //     Add { task_name: String },
    //     List { list_name: String },
    //     SetActive { list_name: String, task_name: String, },
    //     SetInactive {},
    //     SetComplete {},
}

pub struct TaskList {
    pub path: PathBuf,
    pub created: DateTime<Local>,
    pub tasks: Vec<Task>,
    pub elapsed: i64,
}

impl TaskList {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: path,
            created: Local::now(),
            tasks: Vec::new(),
            elapsed: 0,
        }
    }

    pub fn create_list(&self, list_name: PathBuf) {
        if list_name.exists() {
            println!("{} exists. No action taken.", list_name.display());
        } else {
            println!("Creating {}", list_name.display());
            let task_list = File::create(list_name);
        }
    }
    // pub fn add_task() {}
    // pub fn mark_active() {}
    // pub fn mark_inactive() {}
    // pub fn mark_complete() {}
}
