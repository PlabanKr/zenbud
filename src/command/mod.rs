use regex;

use crate::data;

mod add_commands;
mod list_commands;
mod remove_commands;

// main function that handles and parses the commands
pub fn run_command(command_str: String, mut file_data: &mut data::DataFile) {
    let regex_pattern = regex::Regex::new(r#"("[^"]+"|\S+)"#).unwrap();
    // let parts = command_str.split_whitespace();
    let commands: Vec<&str> = regex_pattern.find_iter(&command_str).map(|mat| mat.as_str()).collect();

    if commands.is_empty() {
        println!("command required");
        return;
    }

    match commands[0] {
        "help" => handle_help(),
        "add" => handle_add(&commands, &mut file_data),
        "list" => handle_list(&commands, &mut file_data),
        "remove" => handle_remove(&commands, &mut file_data),
        "done" => handle_done(&commands, &mut file_data),
        _ => println!("did not understand: {} \n type \"help\" for more information", command_str),
    }
}

// help command
fn handle_help() {
    println!("Zenbud - a simple task manager. It manages tasks, hobbies and routines.");
    println!("{:^80}", "add commands");
    println!("add task <task_name> -eta <eta_value>");
    println!("add hobby <hobby_name> -sit <situation_value> -nxt <next_plan_value>");
    println!("add routine <routine_name> -des <description_value> -sit <situation_value>");
    println!("{:^80}", "list commands");
    println!("list task");
    println!("list hobby");
    println!("list routine");
    println!("{:^80}", "remove commands");
    println!("remove task <task_name>");
    println!("remove hobby <hobby_name>");
    println!("remove routine <routine_name>");
    println!("{:^80}", "done commands");
    println!("done <task_name>");
}

// CRUD -> add, list, remove, update
// task, routine, hobby

// functions that handle the primary command types
// currently we have four primary command which are 'add', 'list', 'remove' and 'done'.

fn handle_add(commands: &[&str], mut file_data: &mut data::DataFile) {
    if commands.len() < 2 {
        println!("add command requires more information");
    } else {
        match commands[1] {
            "task" => add_commands::add_new_task(commands, &mut file_data),
            "hobby" => add_commands::add_new_hobby(commands, &mut file_data),
            "routine" => add_commands::add_new_routine(commands, &mut file_data),
            _ => println!("invalid add command"),
        }
    }
}

fn handle_list(commands: &[&str], file_data: &mut data::DataFile) {
    if commands.len() < 2 {
        println!("list command requires more information");
    } else {
        match commands[1] {
            "task" => list_commands::list_all_tasks(file_data),
            "hobby" => list_commands::list_all_hobbies(file_data),
            "routine" => list_commands::list_all_routines(file_data),
            _ => println!("invalid list command"),
        }
    }
}

fn handle_remove(commands: &[&str], file_data: &mut data::DataFile) {
    if commands.len() < 2 {
        println!("remove command requires more information");
    } else {
        match commands[1] {
            "task" => remove_commands::remove_task(commands, file_data),
            "hobby" => remove_commands::remove_hobby(commands, file_data),
            "routine" => remove_commands::remove_routine(commands, file_data),
            _ => println!("invalid remove command"),
        }
    }
}

fn handle_done(commands: &[&str], file_data: &mut data::DataFile) {
    if commands.len() <  2 {
        println!("done command requires the task name");
    } else {
        let task_name = commands[1];
        for task in &mut file_data.tasks {
            if task.name == task_name {
                task.completed = true;
                return;
            }
        }
        println!("no task found with the name: {}", task_name);
    }
}



// helper functions for the add primary command functions

// check_command_length function checks the command length with the required lenght and returns bool value
// if the command length is lesser than the required length then it also prints an error message
fn check_command_length(commands: &[&str], required_length: usize, error_message: &str) -> bool {
    if commands.len() < required_length {
        println!("{}", error_message);
        false
    } else {
        true
    }
}

// check_flag checks the given flag in the command with the expected flag and returns boolean value along with a error msg if the match fails
fn check_flag(commands: &[&str], index: usize, expected_flag: &str) -> bool {
    if commands.get(index) != Some(&expected_flag) {
        println!("expected {} but found {:?}", expected_flag, commands.get(index));
        false
    } else {
        true
    }
}