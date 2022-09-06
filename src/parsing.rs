
pub fn cut_commands(command_string: String) -> Vec<Vec<Vec<String>>>
{
    let commands_split = command_string.split(';');

    let mut commands: Vec<String> = Vec::new();
    for s in commands_split
    {
        commands.push(String::from(s));
    }

    let mut command_tok: Vec<Vec<Vec<String>>> = Vec::new();
    for command in commands
    {
        let dependent_commands_split = command.split("&&");

        let mut dependent_commands: Vec<String> = Vec::new();
        for s in dependent_commands_split
        {
            dependent_commands.push(String::from(s));
        }

        let mut real_commands: Vec<String> = Vec::new();
        for command in dependent_commands
        {
            //     if !command.starts_with("$")
            //     {
            //         real_commands.push(command);
            //         continue;
            //     }

            // let clean:String = clean_env(command);
            // let result:String;
            // result = match env::var(clean.clone())
            // {
            //     Ok(x) => x,
            //     Err(_) => clean,

            // };

            real_commands.push(command);
        }
        let mut temp_vec: Vec<Vec<String>> = Vec::new();
        for dependent_command in real_commands
        {
            let cmdvec_split = dependent_command.split_whitespace();
            let mut cmdvec: Vec<String> = Vec::new();
            for s in cmdvec_split
            {
                cmdvec.push(String::from(s));
            }
            temp_vec.push(cmdvec);
        }
        command_tok.push(temp_vec);
    }
    let tmpcmd = command_tok.to_owned();

    tmpcmd
}
