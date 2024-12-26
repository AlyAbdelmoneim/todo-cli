use crate::tasks::TaskList;
use std::fs;

const DEFAULT_FILE: &str = ".todo.json";

pub fn save_tasks(tasks: &TaskList) {
    let data = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write(DEFAULT_FILE, data).expect("Failed to save tasks to file");
}

pub fn load_tasks() -> TaskList {
    let data = fs::read_to_string(DEFAULT_FILE).unwrap_or_else(|_| String::from("{}"));
    serde_json::from_str(&data).unwrap_or_else(|_| TaskList::new())
}

