use chrono::{DateTime, Local};
use uuid::Uuid;

enum TaskState {
    Active,
    Inactive,
    Complete,
}


pub struct Task {
    id: Uuid,
    name: String,
    state: TaskState,
    date_created: DateTime<Local>,
    date_completed: DateTime<Local>,
    time_spent: i64,
    active_start_time: i64,
}
