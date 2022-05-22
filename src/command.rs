pub struct Command<'a> {
    pub command_name: &'a str,
    pub arguments: Vec<&'a str>,
}

impl Command<'_> {
    pub fn new(input: &str) -> Command {
        let split = input.split_whitespace();
        let command_name = split.clone().next().unwrap();
        let arguments = split.skip(1).collect::<Vec<&str>>();

        Command {
            command_name,
            arguments,
        }
    }
}
