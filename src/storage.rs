use crate::tasks::TaskList;
use std::fs;
use std::path::PathBuf;
use std::env;




pub fn todo_init(){
    let current_dir = env::current_dir().expect("couldn't find current directory");
    let todo_dir = current_dir.join(".todo");

    if(!todo_dir.exists()){
        match fs::create_dir(&todo_dir) {
            Ok(_) => println!("created empty .todo directory"),
            Err(e) => {
                println!("couldn't create .todo directory {}", e);
                return;
            }
        }
    }else{
        println!(".todo directory already exists");
    }

    let todo_file = todo_dir.join("todo.json");

    if(!todo_file.exists()){
        match fs::File::create(&todo_file){
            Ok(_) => println!("todo.json file creaeted successfully"),
            Err(e) => println!("could not create todo.json file {}" ,e)
        }
    }else{
        println!("todo.json file already exists in .todo directory");
    }

}

fn get_default_file_path() -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    // println!("{:?}", current_dir);
    let todo_dir = current_dir.join(".todo");
    // println!("{:?}", todo_dir);
    if !todo_dir.exists() {
        panic!("Current directory is not initialized with 'todo init'");
    }
    todo_dir.join("todo.json")
}

pub fn save_tasks(tasks: &TaskList) {
    let default_file = get_default_file_path();
    let data = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::create_dir_all(default_file.parent().unwrap()).expect("Failed to create .todo directory");
    fs::write(default_file, data).expect("Failed to save tasks to file");
}

pub fn load_tasks() -> TaskList {
    let default_file = get_default_file_path();
    let data = fs::read_to_string(default_file).unwrap_or_else(|_| String::from("{}"));
    serde_json::from_str(&data).unwrap_or_else(|_| TaskList::new())
}

