use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub current_task: String,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub tag: String,
    pub time: u64,
    pub content: String,
    pub done: bool,
    pub children: Vec<Task>,
}
