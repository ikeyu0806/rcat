use clap::{Command, Arg};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let filepath_arg: Arg = Arg::new("filepath")
        .required(true);

    let command: Command = Command::new("rcat")
        .author("Yuki Ikegaya")
        .version("v1.0.0")
        .arg(filepath_arg);
    
    match command.try_get_matches() {
        Ok(m) => {
            if let Some(filepath) = m.get_one::<String>("filepath") {
                match read_file_contents(filepath) {
                    Ok(contents) => println!("{}", contents),
                    Err(e) => eprintln!("Error reading file: {}", e),
                }
            } else {
                println!("Filepath not provided");
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn read_file_contents(filepath: &str) -> io::Result<String> {
    let path = Path::new(filepath);

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
