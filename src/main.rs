mod commands;
mod redirection;

use commands::{change_directory, is_cd_command, Command};
use redirection::{redirect};
use std::process::{Command as ProcessCommand};
use std::{
    env,
    io::{stdout, Write},
};
use text_io::{read};

fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => print!("{}: ", path.display()),
        Err(_) => panic!("An error occurred when printing the working directory!"),
    }

    // flush() forces the buffer to be flushed, causing the content
    // of the buffer to be written to the terminal,
    // even if it normally would wait to do so.
    stdout().flush().unwrap();
}

fn contains_redirection(input: &str) -> bool {
    input.contains('<') || input.contains('>') || input.contains('|')
}

fn main() {
    loop {
        print_working_directory();

        let read: String = read!("{}\n");
        let input = read.trim();

        if is_cd_command(input) {
            let command = Command::new(&input);
            let path = command
                .arguments
                .clone()
                .into_iter()
                .next()
                .unwrap_or_default();

            match change_directory(path) {
                Ok(_) => (),
                Err(error) => println!("{}", error)
            }

            continue;
        }

        if contains_redirection(input) {
            match redirect(input) {
                Ok(_) => (),
                Err(error) => println!("{}", error)
            }

            continue;
        }

        let command = Command::new(&input);

        let command_result = ProcessCommand::new(command.command_name)
            .args(command.arguments)
            .env("PATH", "/bin")
            .output();

        match command_result {
            Ok(output) => match std::str::from_utf8(&output.stdout) {
                Ok(stdout) => println!("{}", stdout),
                Err(error) => println!("{}", error),
            },
            Err(error) => println!("{}", error),
        }
    }
}