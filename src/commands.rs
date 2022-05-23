use home::home_dir;
use std::env::set_current_dir;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::path::Path;

pub struct Command<'a> {
    pub command_name: &'a str,
    pub arguments: Vec<&'a str>,
}

impl Command<'_> {
    pub fn new(input: &str) -> Command {
        let split = input.split_whitespace();
        let command_name = split.clone().next().unwrap_or_default();
        let arguments = split.skip(1).collect::<Vec<&str>>();

        Command {
            command_name,
            arguments,
        }
    }
}

pub fn is_cd_command(command: &str) -> bool {
    command.len() == 2 && command[..2] == "cd"
}

pub fn change_directory(path: &str) -> std::io::Result<()> {
    let home_directory = home_dir().ok_or(Error::from(NotFound))?;
    let home_as_path = home_directory.as_path();

    let directory = match path {
        "~" => home_as_path,
        "" => Path::new("/"),
        _ => Path::new(path),
    };

    set_current_dir(directory)
}
