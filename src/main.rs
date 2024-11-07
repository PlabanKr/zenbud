use std::io::Write;

use reedline::{DefaultPrompt, Reedline, Signal};
use serde::{Deserialize, Serialize};

mod command;

#[derive(Deserialize, Serialize, Debug)]
struct DataFile {
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
    fn new() -> Self {
        DataFile {
            tasks: Vec::new(),
            hobbies: Vec::new(),
            routines: Vec::new()
        }
    }
}

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let file_path = "data.json";
    if !std::path::Path::new(file_path).exists() {
        let mut file = std::fs::File::create(file_path).expect("Unable to create the file");
        let empty_data = DataFile::new();
        let json = serde_json::to_string(&empty_data).expect("Unable to create empty json data");
        file.write_all(json.as_bytes()).expect("Unable to write empty data");
    }
    let mut file_data: DataFile;
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    file_data = serde_json::from_reader(reader).expect("Unable to read json file");
    println!("JSON DATA\n{:?}", file_data);

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                command::run_command(buffer);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}
