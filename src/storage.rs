use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write, Read, Error};
use crate::tasks::{TaskList};
const DEFAULT_FILE: &str = ".todo.json";
pub fn save_to_file(tasks : &TaskList) -> Result<(), Error> {
    let file = File::create(DEFAULT_FILE)?;
    serde_json::to_writer_pretty(file, tasks)?;
    Ok(())
}

pub fn load_from_file() -> Result<TaskList, Error> {
    let file = match File::open(DEFAULT_FILE) {
        Ok(file) => file,
        Err(_) => return Ok(TaskList::new()),
    };
    let mut reader = io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let tasks : TaskList = serde_json::from_str(&contents)?;
    Ok(tasks)
}

