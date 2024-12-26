use clap::{Arg, ArgMatches, Command, ArgGroup};
use crate::tasks::{TaskPriority, TaskList};
use crate::storage::{load_tasks, save_tasks, todo_init};

// Commands

pub fn init_command() -> Command {
    Command::new("init")
        .about("Create the todo repo")
}

pub fn add_command() -> Command {
    Command::new("add")
        .about("Add a new task")
        .arg(Arg::new("title").required(true).short('t').value_parser(clap::value_parser!(String)))
        .arg(Arg::new("description").required(true).short('d').value_parser(clap::value_parser!(String)))
        .arg(Arg::new("priority").short('p').value_parser(clap::value_parser!(TaskPriority)))
}

pub fn remove_command() -> Command {
    Command::new("remove")
        .about("Remove a task")
        .arg(Arg::new("id").required(true).value_parser(clap::value_parser!(u64)))
}

pub fn list_command() -> Command {
    Command::new("list").about("List all tasks")
}

pub fn clear_command() -> Command {
    Command::new("clear")
        .about("Clear tasks")
        .arg(Arg::new("all").short('a').long("all").help("Clear all tasks").num_args(0))
        .arg(Arg::new("done").short('d').help("Clear completed tasks").num_args(0))
        .arg(Arg::new("not-done").short('n').help("Clear incomplete tasks").num_args(0))
        .arg(Arg::new("id").long("id").help("Clear a task by its ID").value_parser(clap::value_parser!(u64)))
        .group(
            ArgGroup::new("clear-options")
                .args(["all", "done", "not-done", "id"])
                .required(true),
        )
}

pub fn alter_command() -> Command {
    Command::new("alter")
        .about("Alter a task")
        .arg(Arg::new("id").required(true).long("id").value_parser(clap::value_parser!(u64)))
        .arg(Arg::new("title").short('t').value_parser(clap::value_parser!(String)))
        .arg(Arg::new("description").short('d').value_parser(clap::value_parser!(String)))
        .arg(Arg::new("priority").short('p').value_parser(clap::value_parser!(TaskPriority)))
        .arg(Arg::new("done").short('D').value_parser(clap::value_parser!(bool)))
}

// Handlers
pub fn handle_init() {
    // Handle the 'init' command to create the .todo directory and tasks file
    todo_init();
    println!("Todo repository initialized.");
}

pub fn handle_add(matches: &ArgMatches) {
    let title = matches.get_one::<String>("title").unwrap().to_string();
    let description = matches.get_one::<String>("description").unwrap().to_string();
    let priority = matches.get_one::<TaskPriority>("priority").cloned().unwrap_or(TaskPriority::Low);
    let mut tasks = load_tasks();
    tasks.add_task(title, description, priority);
    save_tasks(&tasks);
    println!("Task added successfully.");
}

pub fn handle_remove(matches: &ArgMatches) {
    let id = matches.get_one::<u64>("id").unwrap();
    let mut tasks = load_tasks();
    tasks.remove_task(*id);
    save_tasks(&tasks);
    println!("Task removed successfully.");
}

pub fn handle_list() {
    let tasks = load_tasks();
    for task in tasks.all_tasks() {
        task.display();
    }
}

pub fn handle_clear(matches: &ArgMatches) {
    let mut tasks = load_tasks();

    if matches.contains_id("all") {
        tasks.clear_all();
    } else if matches.contains_id("done") {
        tasks.clear_done();
    } else if matches.contains_id("not-done") {
        tasks.clear_undone();
    } else if let Some(id) = matches.get_one::<u64>("id") {
        tasks.remove_task(*id);
    }

    save_tasks(&tasks);
    println!("Tasks cleared successfully.");
}

pub fn handle_alter(matches: &ArgMatches) {
    let id = matches.get_one::<u64>("id").unwrap();
    let mut tasks = load_tasks();

    if let Some(title) = matches.get_one::<String>("title") {
        let _ = tasks.alter_task_title(*id, title.clone());
    }
    if let Some(description) = matches.get_one::<String>("description") {
       let _ =  tasks.alter_task_description(*id, description.clone());
    }
    if let Some(priority) = matches.get_one::<TaskPriority>("priority") {
         let _ = tasks.alter_task_priority(*id, priority.clone());
    }
    if let Some(done) = matches.get_one::<bool>("done") {
        let _ = tasks.alter_task_done(*id, *done);
    }

    save_tasks(&tasks);
    println!("Task altered successfully.");
}

