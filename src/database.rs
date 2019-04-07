use serde::{Deserialize, Serialize};
use sha1::Sha1;

use std::time::{SystemTime, UNIX_EPOCH};

pub enum DBError {}

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
    pub fn add_child_task(&mut self, tag: &str, task: Task) -> Result<String, DBError> {
        unimplemented!();
    }
    pub fn list_tasks(&self) -> Vec<&Task> {
        use std::collections::VecDeque;
        let mut taskq: VecDeque<&Task> = VecDeque::new();
        for task in self.tasks.iter() {
            taskq.push_back(task);
        }
        let mut tasks: Vec<&Task> = Vec::new();
        while !taskq.is_empty() {
            let task = taskq.pop_front().unwrap();
            tasks.push(task);
            for child in task.children.iter().rev() {
                taskq.push_front(child);
            }
        }
        tasks
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
