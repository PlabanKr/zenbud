use regex;

use crate::data;


pub fn run_command(command_str: String, mut file_data: &mut data::DataFile) {
    let regex_pattern = regex::Regex::new(r#"("[^"]+"|\S+)"#).unwrap();
    // let parts = command_str.split_whitespace();
    let commands: Vec<&str> = regex_pattern.find_iter(&command_str).map(|mat| mat.as_str()).collect();

    if commands.is_empty() {
        println!("Command required.");
        return;
    }

    match commands[0] {
        "add" => handle_add(&commands, &mut file_data),
        "list" => handle_list(&commands, &mut file_data),
        "remove" => handle_remove(&commands, &mut file_data),
        _ => println!("Did not understand: {}", command_str),
    }
}

// CRUD -> add, list, remove, update
// task, routine, hobby

fn handle_add(commands: &[&str], mut file_data: &mut data::DataFile) {
    if commands.len() < 2 {
        println!("add command requires more information.");
    } else {
        match commands[1] {
            "task" => add_new_task(commands, &mut file_data),
            "hobby" => add_new_hobby(commands, &mut file_data),
            "routine" => add_new_routine(commands, &mut file_data),
            _ => println!("Invalid add command"),
        }
    }
}

fn handle_list(commands: &[&str], mut file_data: &mut data::DataFile) {
    if commands.len() < 2 {
        println!("list command requires more information.");
    } else {
        match commands[1] {
            "task" => println!("List task commands"),
            "hobby" => println!("List hobbies commands"),
            "routine" => println!("Add routine commands"),
            _ => println!("Invalid list command"),
        }
    }
}

fn handle_remove(commands: &[&str], mut file_data: &mut data::DataFile) {
    if commands.len() < 2 {
        println!("remove command requires more information.");
    } else {
        match commands[1] {
            "task" => println!("Remove task commands"),
            "hobby" => println!("Remove hobbies commands"),
            "routine" => println!("Add routine commands"),
            _ => println!("Invalid remove command"),
        }
    }
}



pub fn add_new_task(commands: &[&str], mut file_data: &mut data::DataFile) {
    if commands.len() < 3 {
        println!("cannot add empty task.");
    } else {
        println!("Add task commands with value {}", commands[2]);
    }
}

pub fn add_new_hobby(commands: &[&str], mut file_data: &mut data::DataFile) {
    if commands.len() < 3 {
        println!("cannot add empty hobby.");
    } else {
        println!("Add hobby commands with value {}", commands[2]);
    }
}

pub fn add_new_routine(commands: &[&str], mut file_data: &mut data::DataFile) {
    if commands.len() < 3 {
        println!("cannot add empty routine.");
    } else {
        println!("Add routine commands with value {}", commands[2]);
    }
}