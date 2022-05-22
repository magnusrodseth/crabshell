use std::{env, io::{stdout, Write}};
use std::iter::Skip;
use std::process::Command as ProcessCommand;
use std::str::SplitWhitespace;
use text_io::read;

fn print_working_directory() {
    let path = env::current_dir();

    match path {
        Ok(_) => print!("{}: ", path.unwrap().display()),
        Err(_) => panic!("An error occurred when printing the working directory!")
    }

    // flush() forces the buffer to be flushed, causing the content
    // of the buffer to be written to the terminal,
    // even if it normally would wait to do so.
    stdout().flush().unwrap();
}

struct Command<'a> {
    command_name: &'a str,
    arguments: Vec<&'a str>,
}

impl Command<'_> {
    fn format(&self) -> String {
        format!("{} {:?}", self.command_name, self.arguments)
    }
}

fn is_valid_user_input(user_input: &str) -> bool {
    // TODO: Implement real validation
    user_input.len() >= 1
}

fn main() {
    print_working_directory();

    let user_input: String = read!("{}\n");

    if !is_valid_user_input(&user_input) {
        // TODO: Handle illegal user input 
    }

    let split = user_input.split_whitespace();
    let command_name = split.clone().next().unwrap();
    let arguments = split.skip(1).collect::<Vec<&str>>();

    let command = Command {
        command_name,
        arguments,
    };

    let out = {
        ProcessCommand::new(command.command_name)
            .args(command.arguments)
            .output()
            .expect("Failed to execute the command!")
    };

    let stdout = out.stdout;

    println!("{}", std::str::from_utf8(&stdout).unwrap());
}
