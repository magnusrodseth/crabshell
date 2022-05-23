use std::env;
use std::io::{stdout, Write};

pub fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => print!("{}: ", path.display()),
        Err(_) => panic!("An error occurred when printing the working directory!"),
    }

    // flush() forces the buffer to be flushed, causing the content
    // of the buffer to be written to the terminal,
    // even if it normally would wait to do so.
    stdout().flush().unwrap();
}
