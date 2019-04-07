use serde::{Deserialize, Serialize};
use sha1::Sha1;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub current_task: String,
    pub tasks: Vec<Task>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            current_task: "".to_owned(),
            tasks: Vec::new(),
        }
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn add_child_task(&mut self, tag: &str, task: Task) {
        unimplemented!();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub tag: String,
    pub time: u64,
    pub content: String,
    pub done: bool,
    pub children: Vec<Task>,
}

impl Task {
    pub fn new(content: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        let tag = Sha1::from(now.to_string() + &content).hexdigest();
        Self {
            tag,
            time: 0,
            content,
            done: false,
            children: Vec::new(),
        }
    }
}
