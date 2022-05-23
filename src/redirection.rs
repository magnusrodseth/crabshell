use std::error::Error;
use std::fs::File;
use std::io::{Write};
use crate::{Command, ProcessCommand};

pub fn redirect(input: &str) -> Result<(), Box<dyn Error>> {
    let redirect_in_index = input.find("<");
    let redirect_out_index = input.find(">");

    if should_redirect_in_and_out(input) {
        // Case: command < file in > file out
    } else if should_redirect_out(input, redirect_out_index) {
        // Case: command > file out
        match redirect_out(input) {
            Ok(_) => (),
            Err(error) => println!("{}", error)
        }
    } else if should_redirect_in(input, redirect_in_index) {
        // Case: command < file in
        match redirect_in(input) {
            Ok(_) => (),
            Err(error) => println!("{}", error)
        }
    }

    Ok(())
}

fn redirect_in(input: &str) -> Result<(), Box<dyn Error>> {
    let mut split = input.split("<");
    let command_input = split.next();
    let filename = split.next();

    // TODO: Change this to check if command_input and filename is valid
    if command_input.is_some() && filename.is_some() {
        let command_input = command_input.unwrap().trim();
        let filename = filename.unwrap().trim();

        let command = Command::new(command_input);
        let file = File::open(filename)?;

        let output = ProcessCommand::new(command.command_name)
            .args(command.arguments)
            // The child process' stdin comes from the opened file
            .stdin(file)
            .output()
            .expect("file error!!");

        let stdout = std::str::from_utf8(&output.stdout)?;
        println!("{}", stdout);
    }

    Ok(())
}

fn redirect_out(input: &str) -> Result<(), Box<dyn Error>> {
    let mut split = input.split(">");
    let command_input = split.next();
    let filename = split.next();

    // TODO: Change this to check if command_input and filename is valid
    if command_input.is_some() && filename.is_some() {
        let command_input = command_input.unwrap().trim();
        let filename = filename.unwrap().trim();

        let command = Command::new(command_input);

        let output = ProcessCommand::new(command.command_name)
            .args(command.arguments)
            .env("PATH", "/bin")
            .output()?;

        let stdout = std::str::from_utf8(&output.stdout)?;

        write_to_file(stdout, filename)?;
    }

    Ok(())
}

fn should_redirect_in(input: &str, in_index: Option<usize>) -> bool {
    in_index.is_some() && appears_once(input, '<')
}

fn should_redirect_out(input: &str, out_index: Option<usize>) -> bool {
    out_index.is_some() && appears_once(input, '>')
}

fn should_redirect_in_and_out(input: &str) -> bool {
    let redirect_in = input.find("<");
    let redirect_out = input.find(">");

    redirect_in.is_some()
        && redirect_out.is_some()
        && in_before_out(redirect_in, redirect_out)
        && appears_once(input, '>')
        && appears_once(input, '<')
}

fn appears_once(string: &str, character: char) -> bool {
    string.matches(character).count() == 1
}

fn in_before_out(in_index: Option<usize>, out_index: Option<usize>) -> bool {
    in_index < out_index
}

fn write_to_file(payload: &str, filename: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(payload.as_ref())?;
    Ok(())
}