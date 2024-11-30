use clap::{Arg, Command};
use crate::cli::{add_task, remove_task, list_tasks};
use crate::tasks::TaskPriority;


fn main() {
    let matches = Command::new("Todo CLI")
        .version("1.0")
        .author("Your Name <aliabubeih@gmail.com>")
        .about("Manage your tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(Arg::new("title").about("Title of the task").required(true))
                .arg(Arg::new("description").about("Description of the task").required(true))
                .arg(Arg::new("priority").about("Priority of the task").required(true))
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a task")
                .arg(Arg::new("id").about("ID of the task").required(true))
        )
        .subcommand(
            Command::new("list")
                .about("List all tasks")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", args)) => {
            let title = args.value_of("title").unwrap();
            let description = args.value_of("description").unwrap();
            let priority = match args.value_of("priority").unwrap() {
                "low" => TaskPriority::Low,
                "medium" => TaskPriority::Medium,
                "high" => TaskPriority::High,
                _ => TaskPriority::Low,
            };
            add_task(title.to_string(), description.to_string(), priority);
        }
        Some(("remove", args)) => {
            let id = args.value_of("id").unwrap().parse().unwrap();
            remove_task(id);
        }
        Some(("list", _)) => {
            list_tasks();
        }
        _ => unreachable!(),
    }
}
