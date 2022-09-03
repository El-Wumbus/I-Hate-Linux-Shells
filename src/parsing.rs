use std::io;

pub fn cut_commands(command_string: &str) -> Vec<Vec<Vec<&str>>> {
    let commands: Vec<&str> = command_string.split(';').collect();
    let mut command_tok: Vec<Vec<Vec<&str>>> = Vec::new();
    for command in commands.iter() {
        let dependent_commands: Vec<&str> = command.split("&&").collect();
        let mut temp_vec: Vec<Vec<&str>> = Vec::new();
        for dependent_command in dependent_commands.iter() {
            temp_vec.push(dependent_command.split_whitespace().collect());
        }
        command_tok.push(temp_vec);
    }
    command_tok
}

pub fn get_cmd() -> String 
{
    let mut command_string:String = String::new();
    io::stdin().read_line(&mut command_string).unwrap();
    if command_string.trim() == ""
    {
        return String::from("");
    }

    command_string
}