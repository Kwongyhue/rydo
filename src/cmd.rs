mod tasks;

use chrono::{DateTime, Duration, Utc};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::prelude::*;
use std::path::PathBuf;
use tasks::{Task, TaskState};
use uuid::Uuid;

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Show {
        list_name: String,
    },
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

    pub fn show_list(&self) -> Result<(), Box<dyn Error>> {
        if !self.path.exists() {
            let err = format!(
                "{} does not exist. Create a list or enter an existing task list.",
                self.path.display()
            );
            Err(err.into())
        } else {
            let mut file = File::open(self.path.as_path())?;
            let mut json_string = String::new();
            file.read_to_string(&mut json_string)?;
            let task_list: TaskList = serde_json::from_str(&json_string)?;
            println!("Rydo {}", task_list.path.display());
            for task in task_list.tasks {
                println!("Task: {}", task.name);
                println!("State: {}", task.state);
                let local_dc = task.date_created.naive_local();
                println!("Created: {local_dc}");
                if let Some(date_completed) = task.date_completed {
                    println!("Completed: {}", date_completed.date_naive());
                };
                println!("Time spent: {}", task.time_spent);
            }
            Ok(())
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
            let task = Task {
                id: Uuid::new_v4(),
                name: task_name,
                state: TaskState::Inactive,
                date_created: Utc::now(),
                date_completed: None,
                time_spent: Duration::zero(),
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
