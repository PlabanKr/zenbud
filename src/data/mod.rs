use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DataFile {
  pub tasks: Vec<Task>,
  pub hobbies: Vec<Hobby>,
  pub routines: Vec<Routine>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
  pub name: String,
  pub completed: bool,
  pub eta: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Hobby {
  pub name: String,
  pub situation: String,
  pub next_plan: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Routine {
  pub name: String,
  pub description: String,
  pub situation: String,
}

impl DataFile {
  pub fn new() -> Self {
    DataFile {
      tasks: Vec::new(),
      hobbies: Vec::new(),
      routines: Vec::new(),
    }
  }
}
