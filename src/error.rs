#[derive(Debug, thiserror::Error)]
pub enum RydoError {
    #[error("Task list {0} not found")]
    ListNotFound(String),
    #[error("Task {0} not found in list {1}")]
    TaskNotFound(String, String),
    #[error("Tasks {0} already exists in list {1}")]
    TaskAlreadyExists(String, String),
}