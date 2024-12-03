mod cli;
mod tasks;

use clap::{Arg, Command, ArgMatches, command};
use crate::tasks::TaskPriority;

fn main() {
    let match_result = command!()
    .about("Testing ")
    .subcommand(
        Command::new("add")
        .about("Add a new task")
                .arg(Arg::new("title"))
                .arg(Arg::new("description"))
                .arg(Arg::new("priority"))
    )
    .subcommand(
        Command::new("remove")
        .about("Remove a task")
                .arg(Arg::new("id"))
                .arg(Arg::new("title"))
    )
    .subcommand(
        Command::new("list")
        .about("List all tasks")
                .arg(Arg::new("title"))
                .arg(Arg::new("priority"))

    )
    .get_matches();

    match match_result.subcommand() {
        Some(("add", match_result)) => add_task_2(match_result),
        Some(("remove", match_result)) => remove_task_2(match_result),
        Some(("list", match_result)) => list_tasks_2(match_result),
        _ => println!("No command provided"),
    }
}
fn add_task_2(match_result : ArgMatches) {
    let title = match_result.get_one::<String>("title").unwrap().to_string();
    let description = match_result.get_one::<String>("description").unwrap().to_string();
    let priority = match_result.get_one::<TaskPriority>("priority").unwrap();
    cli::add_task(title, description, priority);
}
fn remove_task_2(match_result : ArgMatches) {
    let id = match_result.get_one::<u64>("id").unwrap();
    cli::remove_task(*id);
}
fn list_tasks_2(match_result : ArgMatches) {
    cli::list_tasks();
}
