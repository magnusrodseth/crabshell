mod commands;

use std::{env, io::{stdout, Write}};
use commands::{Command, is_cd_command, change_directory};
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


fn main() {
    loop {
        print_working_directory();

        let input: String = read!("{}\n");

        let command = Command::new(&input);

        if is_cd_command(command.command_name) {
            let path = command.arguments
                .clone()
                .into_iter()
                .next()
                .unwrap_or_default();

            let _ = change_directory(path);

            continue;
        }

        let result = ProcessCommand::new(command.command_name)
            .args(command.arguments)
            .output()
            .expect("Failed to execute the command!");

        println!("{}", std::str::from_utf8(&result.stdout).unwrap());
    }
}
