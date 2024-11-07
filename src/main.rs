use reedline::{DefaultPrompt, Reedline, Signal};
use std::io::Write;


mod command;
mod data;

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let file_path = "data.json";
    if !std::path::Path::new(file_path).exists() {
        let mut file = std::fs::File::create(file_path).expect("Unable to create the file");
        let empty_data = data::DataFile::new();
        let json = serde_json::to_string(&empty_data).expect("Unable to create empty json data");
        file.write_all(json.as_bytes()).expect("Unable to write empty data");
    }
    let mut file_data: data::DataFile;
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
                command::run_command(buffer, &mut file_data);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                let data_string = serde_json::to_string(&file_data).expect("Unable to convert file_data to file_string");
                std::fs::write(file_path,data_string.as_bytes()).expect("Unable to write data to json file");
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}
