use crate::data;

pub fn list_all_tasks(file_data: &mut data::DataFile) {
  if file_data.tasks.is_empty() {
      println!("No tasks found.");
      return;
  }

  println!("{:<50} | {:<10} | {:<15}", "Name", "Completed", "ETA");
  println!("{}", "-".repeat(90));

  for task in &file_data.tasks {
      println!(
          "{:<50} | {:<10} | {:<15}",
          task.name,
          task.completed,
          task.eta
      );
  }
}

pub fn list_all_hobbies(file_data: &mut data::DataFile) {
  if file_data.hobbies.is_empty() {
      println!("No hobbies found.");
      return;
  }

  println!("{:<50} | {:<25} | {:<20}", "Name", "Situation", "Next Plan");
  println!("{}", "-".repeat(100));

  for hobby in &file_data.hobbies {
      println!(
          "{:<50} | {:<25} | {:<20}",
          hobby.name,
          hobby.situation,
          hobby.next_plan
      );
  }
}

pub fn list_all_routines(file_data: &mut data::DataFile) {
  if file_data.routines.is_empty() {
      println!("No routines found.");
      return;
  }

  println!("{:<50} | {:<25} | {:<20}", "Name", "Description", "Situation");
  println!("{}", "-".repeat(100));

  for routine in &file_data.routines {
      println!(
          "{:<50} | {:<25} | {:<20}",
          routine.name,
          routine.description,
          routine.situation
      );
  }
}
