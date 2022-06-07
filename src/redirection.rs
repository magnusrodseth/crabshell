use std::error::Error;
use std::fs::File;
use std::io::{Write};
use crate::{Command, ProcessCommand};

pub fn redirect(input: &str) -> Result<(), Box<dyn Error>> {
    let redirect_in_index = input.find("<");
    let redirect_out_index = input.find(">");

    if should_redirect_in_and_out(input) {
        match redirect_in_and_out(input) {
            Ok(_) => (),
            Err(error) => println!("{}", error)
        }
    } else if should_redirect_out(input, redirect_out_index) {
        match redirect_out(input) {
            Ok(_) => (),
            Err(error) => println!("{}", error)
        }
    } else if should_redirect_in(input, redirect_in_index) {
        match redirect_in(input) {
            Ok(_) => (),
            Err(error) => println!("{}", error)
        }
    }

    Ok(())
}



pub fn contains_redirection(input: &str) -> bool {
    input.contains('<') || input.contains('>')
}

fn redirect_in_and_out(input: &str) -> Result<(), Box<dyn Error>> {
    // Case: command < file in > file out
    let mut in_split = input.split("<");
    let command_input = in_split.next();
    let rest = in_split.next();
    let mut out_split = rest.clone().unwrap().split(">");
    let filename_in = out_split.next();
    let filename_out = out_split.next();

    // TODO: Validation
    if command_input.is_some()
        && filename_in.is_some()
        && filename_out.is_some()
    {
        let command_input = command_input.unwrap().trim();
        let filename_in = filename_in.unwrap().trim();
        let filename_out = filename_out.unwrap().trim();

        let command = Command::new(command_input);
        let file_in = File::open(filename_in)?;

        let output = ProcessCommand::new(command.command_name)
            .args(command.arguments)
            // The child process' stdin comes from the opened file
            .stdin(file_in)
            .output()?;

        let stdout = std::str::from_utf8(&output.stdout)?;

        write_to_file(stdout, filename_out)?;
    }

    Ok(())
}

fn redirect_in(input: &str) -> Result<(), Box<dyn Error>> {
    // Case: command < file in
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
            .output()?;

        let stdout = std::str::from_utf8(&output.stdout)?;
        println!("{}", stdout);
    }

    Ok(())
}

fn redirect_out(input: &str) -> Result<(), Box<dyn Error>> {
    // Case: command > file out
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