use clap::{Command, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let filepath_arg: Arg = Arg::new("filepath")
        .required(true);

    let number_arg: Arg = Arg::new("number")
        .short('n')
        .long("number")
        .help("Print line numbers")
        .num_args(0);

    let command: Command = Command::new("rcat")
        .author("Yuki Ikegaya")
        .version("v1.0.0")
        .arg(filepath_arg)
        .arg(number_arg);
    
    match command.try_get_matches() {
        Ok(m) => {
            if let Some(filepath) = m.get_one::<String>("filepath") {
                let print_numbers = m.contains_id("number");
    
                match read_file_contents(filepath, print_numbers) {
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

fn read_file_contents(filepath: &str, print_numbers: bool) -> io::Result<String> {
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if print_numbers {
            contents.push_str(&format!("{:>6}: {}\n", i + 1, line));
        } else {
            contents.push_str(&format!("{}\n", line));
        }
    }

    Ok(contents)
}
