use serde::{Deserialize, Serialize};
use std::str::FromStr;
use colored::*;
// use prettytable::{Table, Row, Cell, format};
use std::fmt::Display;
/// Represents the priority level of a task.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskPriority {
    High,
    Medium,
    Low,
}
impl Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::High => write!(f, "{}", "High".red()),
            TaskPriority::Medium => write!(f, "{}", "Medium".yellow()),
            TaskPriority::Low => write!(f, "{}", "Low".green()),
        }
    }
}

impl FromStr for TaskPriority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" => Ok(TaskPriority::High),
            "medium" => Ok(TaskPriority::Medium),
            "low" => Ok(TaskPriority::Low),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

/// Represents an individual task.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub priority: TaskPriority,
}

impl Task {
    /// Creates a new task.
    pub fn new(id: u64, title: String, description: String, priority: TaskPriority) -> Self {
        Self {
            id,
            title,
            description,
            done: false,
            priority,
        }
    }
        pub fn display(&self) {
        println!("{}", "-----------------------------------".blue());
        println!("{}: {}", "ID".yellow(), self.id);
        println!("{}: {}", "Title".green(), self.title);
        println!("{}: {}", "Description".cyan(), self.description);
        match self.priority {
            TaskPriority::High => println!("{}: {}", "Priority".red(), self.priority),
            TaskPriority::Medium => println!("{}: {}", "Priority".yellow(), self.priority),
            TaskPriority::Low => println!("{}: {}", "Priority".green(), self.priority),
        }
        println!(
            "{}: {}",
            "Done".red(),
            if self.done { "Yes".bold() } else { "No".bold() }
        );
        println!("{}", "-----------------------------------".blue());
}
    /// Updates the title of the task.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Updates the description of the task.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Updates the priority of the task.
    pub fn set_priority(&mut self, priority: TaskPriority) {
        self.priority = priority;
    }

    /// Marks the task as done.
    pub fn mark_done(&mut self) {
        self.done = true;
    }

    /// Marks the task as not done.
    pub fn mark_undone(&mut self) {
        self.done = false;
    }
}

/// Represents a list of tasks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    /// Creates a new, empty task list.
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Adds a new task to the list.
    pub fn add_task(&mut self, title: String, description: String, priority: TaskPriority) {
        let id = self.tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
        self.tasks.push(Task::new(id, title, description, priority));
    }

    /// Removes a task by its ID.
    pub fn remove_task(&mut self, id: u64) {
        self.tasks.retain(|task| task.id != id);
    }

    /// Marks a task as done by its ID.
    pub fn mark_done(&mut self, id: u64) -> Result<(), String> {
        self.modify_task(id, |task| task.mark_done())
    }

    /// Marks a task as not done by its ID.
    pub fn mark_undone(&mut self, id: u64) -> Result<(), String> {
        self.modify_task(id, |task| task.mark_undone())
    }

    /// Retrieves all tasks.
    pub fn all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Retrieves all completed tasks.
//    pub fn done_tasks(&self) -> Vec<&Task> {
//        self.tasks.iter().filter(|task| task.done).collect()
//    }
//
//    /// Retrieves all incomplete tasks.
//    pub fn undone_tasks(&self) -> Vec<&Task> {
//        self.tasks.iter().filter(|task| !task.done).collect()
//    }

    /// Clears all tasks.
    pub fn clear_all(&mut self) {
        self.tasks.clear();
    }

    /// Clears completed tasks.
    pub fn clear_done(&mut self) {
        self.tasks.retain(|task| !task.done);
    }

    /// Clears incomplete tasks.
    pub fn clear_undone(&mut self) {
        self.tasks.retain(|task| task.done);
    }
    pub fn alter_task_title(&mut self, id: u64, title: String) -> Result<(), String> {
        self.modify_task(id, |task| task.set_title(title.clone()))
    }

    /// Updates a task's attributes by its ID.
    pub fn modify_task<F>(&mut self, id: u64, mut update_fn: F) -> Result<(), String>
    where
        F: FnMut(&mut Task),
    {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                update_fn(task);
                Ok(())
            }
            None => Err(format!("Task with ID {} not found", id)),
        }
    }
    pub fn alter_task_description(&mut self, id: u64, description: String) -> Result<(), String> {
        self.modify_task(id, |task| task.set_description(description.clone()))
    }
    pub fn alter_task_priority(&mut self, id: u64, priority: TaskPriority) -> Result<(), String> {
        self.modify_task(id, |task| task.set_priority(priority.clone()))
    }
    pub fn alter_task_done(&mut self, id: u64, done: bool) -> Result<(), String> {
        if done {
            self.mark_done(id)
        } else {
            self.mark_undone(id)
        }
    }
}

