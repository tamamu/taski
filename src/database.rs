use serde::{Deserialize, Serialize};
use sha1::Sha1;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum DBError {
    TagNotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub current_task: String,
    pub tasks: Vec<Box<Task>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            current_task: "".to_owned(),
            tasks: Vec::new(),
        }
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(Box::new(task));
    }
    pub fn add_child_task(&mut self, tag: &str, task: Task) -> Result<(), DBError> {
        let parent = self.get_mut_task_by_tag(tag);
        parent
            .map(|taken| taken.add_child(task))
            .ok_or(DBError::TagNotFound)
    }
    pub fn done_task(&mut self, tag: &str) -> Result<(), DBError> {
        let task = self.get_mut_task_by_tag(tag);
        task.map(|taken| taken.mark_as_done())
            .ok_or(DBError::TagNotFound)
    }
    pub fn done_current_task(&mut self) -> Result<(), DBError> {
        let tag = self.current_task.clone();
        if tag.len() == 0 {
            Ok(())
        } else {
            let task = self.get_mut_task_by_tag(&tag);
            task.map(|taken| taken.mark_as_done())
                .ok_or(DBError::TagNotFound)
        }
    }
    pub fn set_current_task(&mut self, tag: &str) -> Result<(), DBError> {
        let task = self.get_mut_task_by_tag(tag);
        match task {
            Some(_) => {
                self.current_task = tag.to_owned();
                Ok(())
            }
            None => Err(DBError::TagNotFound),
        }
    }
    pub fn get_current_task(&mut self) -> Option<&Box<Task>> {
        let tag = self.current_task.clone();
        if tag.len() == 0 {
            None
        } else {
            let task = self.get_task_by_tag(&tag);
            task.map(|taken| taken)
        }
    }
    fn get_task_by_tag(&self, tag: &str) -> Option<&Box<Task>> {
        use std::collections::VecDeque;
        let mut taskq: VecDeque<&Box<Task>> = VecDeque::new();
        let mut target: Option<&Box<Task>> = None;
        for task in self.tasks.iter() {
            taskq.push_back(task);
        }
        while !taskq.is_empty() {
            let task = taskq.pop_front().unwrap();
            if task.tag == tag {
                target = Some(task);
                break;
            }
            for child in task.children.iter().rev() {
                taskq.push_front(child);
            }
        }
        target
    }
    fn get_mut_task_by_tag(&mut self, tag: &str) -> Option<&mut Box<Task>> {
        use std::collections::VecDeque;
        let mut taskq: VecDeque<&mut Box<Task>> = VecDeque::new();
        let mut target: Option<&mut Box<Task>> = None;
        for task in self.tasks.iter_mut() {
            taskq.push_back(task);
        }
        while !taskq.is_empty() {
            let task = taskq.pop_front().unwrap();
            if task.tag == tag {
                target = Some(task);
                break;
            }
            for child in task.children.iter_mut().rev() {
                taskq.push_front(child);
            }
        }
        target
    }
    pub fn list_tasks(&self) -> Vec<PrintableTask> {
        use std::collections::VecDeque;
        let mut taskq: VecDeque<(&Box<Task>, usize, bool)> = VecDeque::new();
        for task in self.tasks.iter() {
            taskq.push_back((task, 0, false));
        }
        let mut tasks: Vec<PrintableTask> = Vec::new();
        while !taskq.is_empty() {
            let (task, level, done) = taskq.pop_front().unwrap();
            tasks.push(PrintableTask::new(task, level, done));
            for child in task.children.iter().rev() {
                taskq.push_front((child, level + 1, done || task.done));
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
    pub children: Vec<Box<Task>>,
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
    pub fn add_child(&mut self, task: Task) {
        self.children.push(Box::new(task));
    }
    pub fn mark_as_done(&mut self) {
        self.done = true;
    }
}

#[derive(Debug, Clone)]
pub struct PrintableTask<'a> {
    pub task: &'a Box<Task>,
    pub level: usize,
    pub done_parent: bool,
}

impl<'a> PrintableTask<'a> {
    pub fn new(task: &'a Box<Task>, level: usize, done: bool) -> Self {
        Self {
            task: task,
            level: level,
            done_parent: done,
        }
    }
}
