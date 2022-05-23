mod commands;

use commands::{change_directory, is_cd_command, Command};
use std::process::Command as ProcessCommand;
use std::{
    env,
    io::{stdout, Write},
};
use text_io::read;

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

fn main() {
    loop {
        print_working_directory();

        let input: String = read!("{}\n");

        let command = Command::new(&input);

        if is_cd_command(command.command_name) {
            let path = command
                .arguments
                .clone()
                .into_iter()
                .next()
                .unwrap_or_default();

            let _ = change_directory(path);

            continue;
        }

        let command_result = ProcessCommand::new(command.command_name)
            .args(command.arguments)
            .output();

        match command_result {
            Ok(output) => match std::str::from_utf8(&output.stdout) {
                Ok(stdout) => println!("{}", stdout),
                Err(error) => println!("{}", error),
            },
            Err(error) => println!("{}", error)
        }
    }
}
