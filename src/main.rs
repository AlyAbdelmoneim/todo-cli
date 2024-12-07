mod cli;
mod tasks;
mod storage;

use clap::command;

fn main() {
    let matches = command!()
        .about("A simple task manager")
        .subcommand(cli::add_command())
        .subcommand(cli::remove_command())
        .subcommand(cli::list_command())
        .subcommand(cli::clear_command())
        .subcommand(cli::alter_command())
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => cli::handle_add(sub_matches),
        Some(("remove", sub_matches)) => cli::handle_remove(sub_matches),
        Some(("list", _)) => cli::handle_list(),
        Some(("clear", sub_matches)) => cli::handle_clear(sub_matches),
        Some(("alter", sub_matches)) => cli::handle_alter(sub_matches),
        _ => println!("No valid command provided. Use --help for usage information."),
    }
}

