use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: u32,
    pub result: String,
}

