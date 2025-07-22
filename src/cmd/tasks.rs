use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::time::SystemTime;
use strum_macros::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Display, PartialEq)]
pub enum TaskState {
    Active,
    Inactive,
    Complete,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Task {
    #[serde(with = "uuid::serde::urn")]
    pub id: Uuid,
    pub name: String,
    pub state: TaskState,
    pub date_created: DateTime<Utc>,
    pub date_completed: Option<DateTime<Utc>>,
    pub time_spent: Duration,
    pub active_start_time: Option<SystemTime>,
}
