mod tasks;

use chrono::{DateTime, Utc};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use tasks::{Task, TaskState};
use uuid::Uuid;

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Create {
        list_name: String,
    },
    Add {
        list_name: String,
        task_name: String,
    },
    //     List { list_name: String },
    //     SetActive { list_name: String, task_name: String, },
    //     SetInactive {},
    //     SetComplete {},
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub path: PathBuf,
    pub created: DateTime<Utc>,
    pub tasks: Vec<Task>,
    pub elapsed: i64,
}

impl TaskList {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            created: Utc::now(),
            tasks: Vec::new(),
            elapsed: 0,
        }
    }

    pub fn create_list(&self) -> Result<(), Box<dyn Error>> {
        if self.path.exists() {
            println!("{} exists. No action taken.", self.path.display());
        } else {
            println!("Creating {}", self.path.display());
            let mut file = File::create(self.path.as_path())?;
            let list_json = serde_json::to_string_pretty(self).unwrap();
            let _ = file.write_all(list_json.as_bytes());
        }
        Ok(())
    }
    pub fn add_task(&mut self, task_name: String) -> Result<(), Box<dyn Error>> {
        if !self.path.exists() {
            let err_msg = format!(
                "{} does not exist. First create a list with rydo create <list_name>",
                self.path.display(),
            );
            Err(err_msg.into())
        } else {
            // let mut file = File::open(self.path.as_path())?;
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(self.path.as_path())?;
            let task = Task{
                id: Uuid::new_v4(),
                name: task_name,
                state: TaskState::Inactive,
                date_created: Utc::now(),
                date_completed: None,
                time_spent: 0,
                active_start_time: None,
            };
            self.tasks.push(task);
            let task_list_json = serde_json::to_string_pretty(self)?;
            file.write_all(task_list_json.as_bytes())?;
            Ok(())
        }
    }
    // pub fn mark_active() {}
    // pub fn mark_inactive() {}
    // pub fn mark_complete() {}
}
