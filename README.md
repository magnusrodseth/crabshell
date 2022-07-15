# Crab shell: A simple shell 🦀

## Description ✏️

Crabshell is a shell with simple Unix-like functionality, written in Rust. 

The idea for the project was formed after creating a similar shell in C for an operating systems course at NTNU, sparking some interest in systems programming.

For more information about the shell written in C, see my [**flush** project on GitHub](https://github.com/magnusrodseth/flush).

Crabshell supports the following:

- Running executables in your `PATH`, for instance `ls`, `cd`, `grep`, etc...
- I/O redirection
- Background tasks, with a custom implementation of the `jobs` command
- Pipelines

See examples below.

## Developer Information 🙋‍♂️

Developed by Magnus Rødseth, Summer 2022.

## Running the application ✅

```sh
# Navigate to the project directory
cd crabshell

# Compile and run the application
cargo run

# Test the application
cargo test
```
