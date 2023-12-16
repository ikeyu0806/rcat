use clap::{Command, Arg};

fn main() {
    let filepath_arg: Arg = Arg::new("filepath")
        .required(true);

    let command: Command = Command::new("rcat")
        .author("Yuki Ikegaya")
        .version("v1.0.0")
        .arg(filepath_arg);
    
    match command.try_get_matches() {
        Ok(m) => {
            let filepath = m.get_one::<String>("filepath").unwrap();

            println!("{}", filepath);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
