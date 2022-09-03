mod parsing;
mod defines;
mod ansi;

extern crate libc;
use std::env;
use std::io;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::path::Path;
use parsing::{cut_commands, get_cmd};
use std::process::Command;
use users::{get_current_uid, get_user_by_uid};
use defines::{PROGRAM_NAME};

#[allow(non_snake_case)]
use ansi::Ansi;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// admin_level to consider
    #[structopt(short, long)]
    command: String,
}

// fn help() {

//     println!("usage:
// -c | --command <string>
//     Run command without interactive prompts.");
// }


fn main() 
{
    let mut interactive: bool = true;
    let mut repeat_once:bool = false;
    let opt = Opt::from_args();

    if opt.command == ""
    {
        interactive = false;
        repeat_once = true;
    }
    
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
        let input:String;
        if !interactive 
        {
            print!("{}", command_prompt);
            io::stdout().flush().expect("ihls:: Couldn't flush stdout");
            input = get_cmd();
        }
        else
        {
            // Strings are pointers, the actual data doesn't get auto copied so we clone it.
            input = String::clone(&opt.command); 
        }

        /* Ensure to skip lines that are empty so the program
        doesn't panic. */
        if input.trim() == "" 
        {
            continue;
        }

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
        if repeat_once != true
        {
            break;
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
        
        printerror(format!("{}:: Command not found!", PROGRAM_NAME));
        return true // bad
    }
}

fn change_dir(path: &str) -> bool{
    let path = Path::new(path);
    match env::set_current_dir(&path) {
        Err(err) => {
            printerror(format!("{}:: Failed to change the directory!\n{}",PROGRAM_NAME , err));
            return false;
        }
        _ => (),
    }
    return true;
}

pub fn printerror(string: String)
{
  eprintln!("{}{}{}",Ansi::RED,string,Ansi::COLOR_END);
}