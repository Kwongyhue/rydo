use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
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
    pub time_spent: i64,
    pub active_start_time: Option<i64>,
}
