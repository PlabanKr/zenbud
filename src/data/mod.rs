use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DataFile {
  tasks: Vec<Task>,
  hobbies: Vec<Hobby>,
  routines: Vec<Routine>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
  name: String,
  completed: bool,
  eta: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Hobby {
  name: String,
  situation: String,
  next_plan: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Routine {
  name: String,
  description: String,
  situation: String,
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
