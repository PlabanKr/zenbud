use crate::data;

pub fn add_new_task(commands: &[&str], file_data: &mut data::DataFile) {
  if !super::check_command_length(commands, 3, "cannot add empty task") {
      return;
  }

  let task_name = commands[2].to_string();
  let eta = if commands.len() > 3 {
      if !super::check_flag(commands, 3, "-eta") || !super::check_command_length(commands, 5, "no eta value was provided") {
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
  if !super::check_command_length(commands, 3, "cannot add empty hobby") {
      return;
  }

  let hobby_name = commands[2].to_string();
  let situation = if commands.len() > 3 {
      if !super::check_flag(commands, 3, "-sit") || !super::check_command_length(commands, 5, "no situation value was provided") {
          return;
      }
      commands[4].to_string()
  } else {
      String::new()
  };

  let next_plan = if commands.len() > 5 {
      if !super::check_flag(commands, 5, "-nxt") || !super::check_command_length(commands, 7, "no next_plan value was provided") {
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
  if !super::check_command_length(commands, 3, "cannot add empty routine") {
      return;
  }

  let routine_name = commands[2].to_string();
  let description = if commands.len() > 3 {
      if !super::check_flag(commands, 3, "-des") || !super::check_command_length(commands, 5, "no description value was provided") {
          return;
      }
      commands[4].to_string()
  } else {
      String::new()
  };

  let situation = if commands.len() > 5 {
      if !super::check_flag(commands, 5, "-sit") || !super::check_command_length(commands, 7, "no situation value was provided") {
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
