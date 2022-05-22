use std::{env, io::{stdout, Write}};
use std::process::Command as ProcessCommand;
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

fn main() {
    print_working_directory();

    let user_input: String = read!("{}\n");

    let split = user_input.split_whitespace();
    let command_name = split.clone().next().unwrap();
    let arguments = split.skip(1).collect::<Vec<&str>>();

    let command = Command {
        command_name,
        arguments,
    };

    let result = {
        ProcessCommand::new(command.command_name)
            .args(command.arguments)
            .output()
            .expect("Failed to execute the command!")
    };

    println!("{}", std::str::from_utf8(&result.stdout).unwrap());
}
