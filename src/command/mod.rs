use regex;

use crate::data;


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
        "add" => handle_add(&commands, &mut file_data),
        "list" => handle_list(&commands, &mut file_data),
        "remove" => handle_remove(&commands, &mut file_data),
        _ => println!("did not understand: {}", command_str),
    }
}

// CRUD -> add, list, remove, update
// task, routine, hobby

// functions that handle the primary command types
// currently we have only three primary command which are 'add', 'list' and 'remove'.

fn handle_add(commands: &[&str], mut file_data: &mut data::DataFile) {
    if commands.len() < 2 {
        println!("add command requires more information");
    } else {
        match commands[1] {
            "task" => add_new_task(commands, &mut file_data),
            "hobby" => add_new_hobby(commands, &mut file_data),
            "routine" => add_new_routine(commands, &mut file_data),
            _ => println!("invalid add command"),
        }
    }
}

fn handle_list(commands: &[&str], file_data: &mut data::DataFile) {
    if commands.len() < 2 {
        println!("list command requires more information");
    } else {
        match commands[1] {
            "task" => list_all_tasks(file_data),
            "hobby" => list_all_hobbies(file_data),
            "routine" => list_all_routines(file_data),
            _ => println!("invalid list command"),
        }
    }
}

fn handle_remove(commands: &[&str], file_data: &mut data::DataFile) {
    if commands.len() < 2 {
        println!("remove command requires more information");
    } else {
        match commands[1] {
            "task" => remove_task(commands, file_data),
            "hobby" => remove_hobby(commands, file_data),
            "routine" => remove_routine(commands, file_data),
            _ => println!("invalid remove command"),
        }
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

// functions that handles the add primary commands

pub fn add_new_task(commands: &[&str], file_data: &mut data::DataFile) {
    if !check_command_length(commands, 3, "cannot add empty task") {
        return;
    }

    let task_name = commands[2].to_string();
    let eta = if commands.len() > 3 {
        if !check_flag(commands, 3, "-eta") || !check_command_length(commands, 5, "no eta value was provided") {
            return;
        }
        commands[4].to_string()
    } else {
        String::new()
    };

    let new_task = data::Task {
        name: task_name,
        completed: false,
        eta,
    };
    file_data.tasks.push(new_task);
}

pub fn add_new_hobby(commands: &[&str], file_data: &mut data::DataFile) {
    if !check_command_length(commands, 3, "cannot add empty hobby") {
        return;
    }

    let hobby_name = commands[2].to_string();
    let situation = if commands.len() > 3 {
        if !check_flag(commands, 3, "-sit") || !check_command_length(commands, 5, "no situation value was provided") {
            return;
        }
        commands[4].to_string()
    } else {
        String::new()
    };

    let next_plan = if commands.len() > 5 {
        if !check_flag(commands, 5, "-nxt") || !check_command_length(commands, 7, "no next_plan value was provided") {
            return;
        }
        commands[6].to_string()
    } else {
        String::new()
    };

    let new_hobby = data::Hobby {
        name: hobby_name,
        situation,
        next_plan,
    };
    file_data.hobbies.push(new_hobby);
}

pub fn add_new_routine(commands: &[&str], file_data: &mut data::DataFile) {
    if !check_command_length(commands, 3, "cannot add empty routine") {
        return;
    }

    let routine_name = commands[2].to_string();
    let description = if commands.len() > 3 {
        if !check_flag(commands, 3, "-des") || !check_command_length(commands, 5, "no description value was provided") {
            return;
        }
        commands[4].to_string()
    } else {
        String::new()
    };

    let situation = if commands.len() > 5 {
        if !check_flag(commands, 5, "-sit") || !check_command_length(commands, 7, "no situation value was provided") {
            return;
        }
        commands[6].to_string()
    } else {
        String::new()
    };

    let new_routine = data::Routine {
        name: routine_name,
        description,
        situation,
    };
    file_data.routines.push(new_routine);
}



// functions that handles the list primary commands

fn list_all_tasks(file_data: &mut data::DataFile) {
    if file_data.tasks.is_empty() {
        println!("no task found");
    }
    for task in &file_data.tasks {
        println!("Name:{}\t|\tCompleted:{}\t|\tETA:{}", task.name, task.completed, task.eta)
    }
}

fn list_all_hobbies(file_data: &mut data::DataFile) {
    if file_data.hobbies.is_empty() {
        println!("no hobby found");
    }
    for hobby in &file_data.hobbies {
        println!("Name:{}\t|\tSituation:{}\t|\tNext Plan:{}", hobby.name, hobby.situation, hobby.next_plan)
    }
}

fn list_all_routines(file_data: &mut data::DataFile) {
    if file_data.routines.is_empty() {
        println!("no routine found");
    }
    for routine in &file_data.routines {
        println!("Name:{}\t|\tSituation{}\t|\tDescription:{}", routine.name, routine.situation, routine.description)
    }
}



// functions that handles the remove primary commands

fn remove_task(commands: &[&str], file_data: &mut data::DataFile) {
    if file_data.tasks.is_empty() {
        println!("task list is empty");
        return;
    }

    if commands.len() < 3 {
        println!("specify the task name to remove");
        return;
    } else if commands[2] == "-idx" {
        if commands.len() < 4 {
            println!("no index were given")
        } else {
            let index: usize = commands[3].parse().unwrap();
            file_data.tasks.remove(index);
        }
    } else {
        let task_name = commands[2];
        file_data.tasks.retain(|ele| ele.name != task_name);
    }
}

fn remove_hobby(commands: &[&str], file_data: &mut data::DataFile) {
    if file_data.tasks.is_empty() {
        println!("hobby list is empty");
        return;
    }

    if commands.len() < 3 {
        println!("specify the hobby name to remove");
        return;
    } else if commands[2] == "-idx" {
        if commands.len() < 4 {
            println!("no index were given")
        } else {
            let index: usize = commands[3].parse().unwrap();
            file_data.hobbies.remove(index);
        }
    } else {
        let hobby_name = commands[2];
        file_data.hobbies.retain(|ele| ele.name != hobby_name);
    }
}

fn remove_routine(commands: &[&str], file_data: &mut data::DataFile) {
    if file_data.tasks.is_empty() {
        println!("routine list is empty");
        return;
    }

    if commands.len() < 3 {
        println!("specify the routine name to remove");
        return;
    } else if commands[2] == "-idx" {
        if commands.len() < 4 {
            println!("no index were given")
        } else {
            let index: usize = commands[3].parse().unwrap();
            file_data.routines.remove(index);
        }
    } else {
        let routine_name = commands[2];
        file_data.routines.retain(|ele| ele.name != routine_name);
    }
}