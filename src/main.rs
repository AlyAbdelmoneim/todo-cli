mod cli;
mod tasks;
mod storage;

use clap::command;

fn main() {
    let matches = command!()
        .about("A simple task manager")
        .subcommand(cli::init_command())
        .subcommand(cli::add_command())
        .subcommand(cli::remove_command())
        .subcommand(cli::list_command())
        .subcommand(cli::clear_command())
        .subcommand(cli::alter_command())
        .get_matches();

    match matches.subcommand() {
        // Handle "init" to initialize the todo repository
        Some(("init", _)) => {
            storage::todo_init(); // Create the .todo directory and tasks file
            println!("Todo repository initialized.");
        }

        // Handle "add" to add a new task
        Some(("add", sub_matches)) => cli::handle_add(sub_matches),

        // Handle "remove" to remove a task
        Some(("remove", sub_matches)) => cli::handle_remove(sub_matches),

        // Handle "list" to display all tasks
        Some(("list", _)) => cli::handle_list(),

        // Handle "clear" to clear tasks based on the options
        Some(("clear", sub_matches)) => cli::handle_clear(sub_matches),

        // Handle "alter" to modify a task
        Some(("alter", sub_matches)) => cli::handle_alter(sub_matches),

        // If no valid command is provided, show the usage information
        _ => println!("No valid command provided. Use --help for usage information."),
    }
}

