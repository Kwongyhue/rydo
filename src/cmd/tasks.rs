use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
enum TaskState {
    Active,
    Inactive,
    Complete,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Task {
    #[serde(with = "uuid::serde::urn")]
    id: Uuid,
    name: String,
    state: TaskState,
    date_created: DateTime<Utc>,
    date_completed: DateTime<Utc>,
    time_spent: i64,
    active_start_time: i64,
}
