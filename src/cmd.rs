mod tasks;

use chrono::{DateTime, Duration, Local, TimeDelta, Utc};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::cmp::{max, min};
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
    SetActive {
        list_name: String,
        task_name: String,
    },
    SetInactive {
        list_name: String,
        task_name: String,
    },
    MarkComplete {
        list_name: String,
        task_name: String,
    },
    AddUrl {
        list_name: String,
        task_name: String,
        url: String,
    },
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub created: DateTime<Utc>,
    pub tasks: Vec<Task>,
    pub total_time_spent: Duration,
}

pub struct TaskListManager {
    pub path: PathBuf,
}

impl TaskListManager {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn show_list(&self) -> Result<(), Box<dyn Error>> {
        let task_list = self.read_task_file()?;
        println!("Rydo {}", self.path.display());
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

    pub fn create_list(&self) -> Result<(), Box<dyn Error>> {
        if self.path.exists() {
            println!("{} exists. No action taken.", self.path.display());
        } else {
            println!("Creating {}", self.path.display());
            let mut file = File::create(self.path.as_path())?;
            let task_list = TaskList {
                created: Utc::now(),
                tasks: Vec::new(),
                total_time_spent: Duration::zero(),
            };
            let list_json = serde_json::to_string_pretty(&task_list).unwrap();
            let _ = file.write_all(list_json.as_bytes());
        }
        Ok(())
    }
    pub fn add_task(&mut self, task_name: String) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(self.path.as_path())?;
        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;
        let mut task_list: TaskList = serde_json::from_str(json_string.as_str())?;
        for task in task_list.tasks.iter() {
            if task.name == task_name {
                let err_msg = format!("{task_name} already exists. Choose a different task name");
                return Err(err_msg.into());
            }
        }
        let task = Task {
            id: Uuid::new_v4(),
            name: task_name,
            state: TaskState::Inactive,
            date_created: Utc::now(),
            date_completed: None,
            time_spent: Duration::zero(),
            active_start_time: None,
            ticket_url: None,
        };
        task_list.tasks.push(task);
        self.write_to_file(task_list)?;
        Ok(())
    }

    pub fn set_active(&self, task_name: String) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(self.path.as_path())?;
        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;
        let mut task_list: TaskList = serde_json::from_str(&json_string)?;
        let mut found_index = None;
        let mut active_index = None;
        for (index, task) in task_list.tasks.iter().enumerate() {
            if task.state == TaskState::Active {
                active_index = Some(index);
            }
            if task.name == task_name {
                found_index = Some(index);
            }
        }
        if let (Some(found), Some(active)) = (found_index, active_index) {
            if found != active {
                let (low, high) = (min(found, active), max(found, active));
                let (first, second) = task_list.tasks.split_at_mut(high);

                let first_task = &mut first[low];
                let second_task = &mut second[0];

                if found < active {
                    first_task.state = TaskState::Active;
                    first_task.active_start_time = Some(Local::now().to_utc());
                    second_task.state = TaskState::Inactive;
                    second_task.active_start_time = None;
                } else {
                    first_task.state = TaskState::Inactive;
                    first_task.active_start_time = None;
                    second_task.state = TaskState::Active;
                    second_task.active_start_time = Some(Local::now().to_utc());
                }
                println!("{}", first_task.state);
                println!("{}", second_task.state);
            }
        } else if let Some(found) = found_index {
            let found_task = &mut task_list.tasks[found];
            found_task.state = TaskState::Active;
            found_task.active_start_time = Some(Local::now().to_utc());
        } else {
            let err_msg = format!("{task_name} task name does not exist");
            return Err(err_msg.into());
        }
        self.write_to_file(task_list)?;
        Ok(())
    }

    pub fn set_inactive(&self, task_name: String) -> Result<(), Box<dyn Error>> {
        let mut task_list = self.read_task_file()?;
        for task in task_list.tasks.iter_mut() {
            if task.name == task_name {
                if task.state == TaskState::Inactive || task.state == TaskState::Complete {
                    let err_msg = format!("Task {} is {}", task.name, task.state);
                    return Err(err_msg.into());
                }
                if let Some(start_time) = task.active_start_time {
                    let diff = Local::now().to_utc().signed_duration_since(start_time);
                    let delta = TimeDelta::seconds(diff.num_seconds());
                    task.time_spent += delta;
                    task.state = TaskState::Inactive;
                    task_list.total_time_spent += delta;
                } else {
                    return Err("Start time not present in active task".into());
                };
            }
        }
        self.write_to_file(task_list)?;
        Ok(())
    }

    pub fn mark_complete(&self, task_name: String) -> Result<(), Box<dyn Error>> {
        let mut task_list = self.read_task_file()?;
        for task in task_list.tasks.iter_mut() {
            if task.name == task_name {
                if task.state == TaskState::Active
                    && let Some(start_time) = task.active_start_time
                {
                    let diff = Local::now().to_utc().signed_duration_since(start_time);
                    let delta = TimeDelta::seconds(diff.num_seconds());
                    task.time_spent += delta;
                    task.active_start_time = None;
                    task_list.total_time_spent += delta;
                }
                task.state = TaskState::Complete;
            }
        }
        self.write_to_file(task_list)?;
        Ok(())
    }

    pub fn add_url(&self, task_name: String, url: String) -> Result<(), Box<dyn Error>> {
        let mut task_list = self.read_task_file()?;
        for task in task_list.tasks.iter_mut() {
            if task.name == task_name {
                task.ticket_url = Some(url.clone());
                self.write_to_file(task_list)?;
                return Ok(());
            }
        }
        let err_msg = format!("{task_name} not found. Cannot add url");
        Err(err_msg.into())
    }

    // Helper functions
    fn read_task_file(&self) -> Result<TaskList, Box<dyn Error>> {
        if !self.path.exists() {
            let err_msg = format!(
                "{} does not exist. First create a list with rydo create <list_name>",
                self.path.display(),
            );
            return Err(err_msg.into());
        }
        let mut file = OpenOptions::new().read(true).open(self.path.as_path())?;
        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;
        let task_list: TaskList = serde_json::from_str(json_string.as_str())?;
        Ok(task_list)
    }

    fn write_to_file(&self, task_list: TaskList) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.path.as_path())?;
        let task_list_json = serde_json::to_string_pretty(&task_list)?;
        file.write_all(task_list_json.as_bytes())?;
        Ok(())
    }
}
