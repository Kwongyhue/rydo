mod tasks;

use chrono::{DateTime, Utc};
use clap::Subcommand;
use serde::Serialize;
use serde_with::serde_as;
use std::error::Error;
use std::fs::File;
use std::io::Write;
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

#[serde_as]
#[derive(Serialize)]
pub struct TaskList {
    pub path: PathBuf,
    pub created: DateTime<Utc>,
    pub tasks: Vec<Task>,
    pub elapsed: i64,
}

impl TaskList {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: path,
            created: Utc::now(),
            tasks: Vec::new(),
            elapsed: 0,
        }
    }

    pub fn create_list(&self, list_name: PathBuf) -> Result<(), Box<dyn Error>> {
        if list_name.exists() {
            println!("{} exists. No action taken.", list_name.display());
        } else {
            println!("Creating {}", list_name.display());
            let mut task_list = File::create(list_name)?;
            let list_json = serde_json::to_string(self).unwrap();
            println!("{}", list_json);
            task_list.write_all(list_json.as_bytes());
        }
        Ok(())
    }
    // pub fn add_task() {}
    // pub fn mark_active() {}
    // pub fn mark_inactive() {}
    // pub fn mark_complete() {}
}
