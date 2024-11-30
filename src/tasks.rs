use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}
impl Default for TaskPriority {
    fn default() -> Self { TaskPriority::Low }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id : u64,
    pub title : String,
    pub description : String,
    pub done : bool,
    pub priority : TaskPriority,
}
impl Task {
    pub fn new(id : u64, title : String, description : String, priority : TaskPriority) -> Self {
        Task {
            id,
            title,
            description,
            done : false,
            priority,
        }
    }
    pub fn defualt() -> Self {
        Task {
            id : 0,
            title : String::from(""),
            description : String::from(""),
            done : false,
            priority : TaskPriority::Low,
        }
    }
    pub fn set_title(&mut self, title : String) {
        self.title = title;
    }
    pub fn set_description(&mut self, description : String) {
        self.description = description;
    }
    pub fn set_priority(&mut self, priority : TaskPriority) {
        self.priority = priority;
    }
    pub fn mark_done(&mut self) {
        self.done = true;
    }
}
pub struct TaskList {
    pub tasks : Vec<Task>,
}
impl TaskList {
    fn new() -> Self {
        TaskList {
            tasks : Vec::new(),
        }
    }
    fn add_task(&mut self, task : Task) {
        self.tasks.push(task);
    }
    fn remove_task(&mut self, id : u64) {
        self.tasks.retain(|task| task.id != id);
    }
    fn get_task(&self, id : u64) -> Task {
        self.tasks.iter().find(|task| task.id == id).unwrap_or(&Task::defualt()).clone()
    }
    fn get_all_tasks(&self) -> Vec<Task> {
        self.tasks.clone()
    }
    fn get_done_tasks(&self) -> Vec<Task> {
        self.tasks.iter().filter(|task| task.done).cloned().collect()
    }
    fn get_undone_tasks(&self) -> Vec<Task> {
        self.tasks.iter().filter(|task| !task.done).cloned().collect()
    }
    fn mark_done(&mut self, id : u64)-> Result<(), String> {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.mark_done();
                Ok(())
            },
            None => Err(String::from("Task not found")),
        }
    }
    fn mark_undone(&mut self, id : u64) -> Result<(), String> {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.done = false;
                Ok(())
            },
            None => Err(String::from("Task not found")),
        }
    }
    fn mark_all_done(&mut self) {
        self.tasks.iter_mut().for_each(|task| task.mark_done());
    }
    fn mark_all_undone(&mut self) {
        self.tasks.iter_mut().for_each(|task| task.done = false);
    }
    fn clear_all_tasks(&mut self) {
        self.tasks.clear();
    }
    fn clear_done_tasks(&mut self) {
        self.tasks.retain(|task| !task.done);
    }
    fn clear_undone_tasks(&mut self) {
        self.tasks.retain(|task| task.done);
    }
    fn clear_task(&mut self, id : u64) {
        self.tasks.retain(|task| task.id != id);
    }
    fn clear_tasks(&mut self, ids : Vec<u64>) {
        self.tasks.retain(|task| !ids.contains(&task.id));
    }
    fn clear_all(&mut self) {
        self.tasks.clear();
    }

}
