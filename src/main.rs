mod splitting;
mod defines;

extern crate libc;
use std::env;
use std::io;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::path::Path;
use splitting::cut_commands;
use std::process::Command;
use users::{get_current_uid, get_user_by_uid};
use defines::{PROGRAM_NAME};



fn main() 
{
    /* Ensure the shell doesn't quit when trying to kill programs
    ruuning within the shell */
    unsafe 
    {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
        libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    }

    // Get the Username
    let user = get_user_by_uid(get_current_uid()).unwrap();

    let mut command_prompt: String;

    loop 
    {
        // The main loop

        command_prompt = String::from(format!("{} > ", user.name().to_string_lossy()));
        print!("{}", command_prompt);
        io::stdout().flush().expect("ihls:: Couldn't flush stdout");
        // let mut input: String = String::new();
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("ihls:: Couldn't read from stdin");

        let input:String = get_cmd();

        /* Ensure to skip lines that are empty so the program
        doesn't panic. */
        if input.trim() == "" 
        {
            continue;
        }
        // let mut parts = input.trim().split_whitespace();
        // let command = parts.next().unwrap();
        // let args = parts;
        let commands = cut_commands(&input);

        for command in commands
        {
            for mut dependent_command in command
            {
                let mut is_background = false;
                if let Some(&"&") = dependent_command.last()
                {
                    is_background = true;
                    dependent_command.pop();
                }
                match dependent_command[0]
                {
                    "exit" =>
                    {
                        std::process::exit(0);
                    },
                    "cd" =>
                    {
                        change_dir(dependent_command[1]);
                    }
                    _ =>
                    {
                        run_cmd(dependent_command, is_background);
                    }
                }
                
            }
        }
        
    }
}

fn run_cmd(command_tok: Vec<&str>, background:bool) -> bool
{
    unsafe 
    {
        let mut command_instance = Command::new(command_tok[0]);

        if let Ok(mut child) = command_instance.args(&command_tok[1..]).pre_exec(|| 
                {
                        libc::signal(libc::SIGINT, libc::SIG_DFL);
                        libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                        Result::Ok(())
                }
            ).spawn()
        {
            if !background
            {
                return child.wait().expect("Command did not run!").success();
            }
            
            println!("{}:: {} working...", PROGRAM_NAME, child.id());
            return false // good
        }
        
        eprintln!("{}:: Command not found!", PROGRAM_NAME);
        return true // bad
    }
}

fn change_dir(path: &str) -> bool{
    let path = Path::new(path);
    match env::set_current_dir(&path) {
        Err(err) => {
            eprintln!("{}:: Failed to change the directory!\n{}",PROGRAM_NAME , err);
            return false;
        }
        _ => (),
    }
    return true;
}


fn get_cmd() -> String 
{
    let mut command_string:String = String::new();
    io::stdin().read_line(&mut command_string).unwrap();
    if command_string.trim() == ""
    {
        return String::from("");
    }

    command_string
}