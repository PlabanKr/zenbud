use crate::data;

pub fn remove_task(commands: &[&str], file_data: &mut data::DataFile) {
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

pub fn remove_hobby(commands: &[&str], file_data: &mut data::DataFile) {
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

pub fn remove_routine(commands: &[&str], file_data: &mut data::DataFile) {
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