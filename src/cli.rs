use clap::{Arg, Command};
use serde_json;
use std::fs::File;
use std::io::{self, BufReader, Read};
use crate::storage::{save_to_file, load_from_file};
use crate::tasks::{TaskList, Task, TaskPriority};

const DEFAULT_FILE: &str = ".todo.json";

pub fn add_task(title : String, description : String, priority :TaskPriority) {
    let mut tasks = load_from_file().unwrap();
    let id = tasks.tasks.len() as u64 + 1;
    let task = Task::new(id, title, description, priority);
    tasks.tasks.push(task);
    save_to_file(&tasks).unwrap();
}

pub fn remove_task (id : u64) {
    let mut tasks = load_tasks();
    tasks.remove_task(id);
    save_to_file(&tasks).unwrap();
}
pub fn list_tasks() {
    let tasks = load_tasks();
    for task in tasks.tasks.get_all_tasks() {
        println!("{}: {} - {} - Priority: {:?} - Done: {}",
                 task.id, task.title, task.description, task.priority, task.done);
    }
}
pub fn load_tasks() -> TaskList {
    load_from_file().unwrap_or_else(|_| TaskList { tasks: Vec::new() })
}

