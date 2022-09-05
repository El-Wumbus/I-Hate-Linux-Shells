mod ansi;
#[macro_use]
mod builtins;
mod defines;
mod parsing;
extern crate libc;
#[allow(non_snake_case)]
use ansi::Ansi;
use defines::PROGRAM_NAME;
use parsing::cut_commands;
use std::env;
use std::io::{stdin, stdout, Write};
use std::os::unix::process::CommandExt;
use std::process::Command;
use structopt::StructOpt;
use users::{get_current_uid, get_user_by_uid};

fn main()
{
    //// let home:String;
    //// match env::var("HOME")
    //// {
    ////     Ok(x) => home = x.clone(),
    ////     Err(_) => home = String::from("/"),
    //// }

    let mut _interactive: bool = false;
    let mut script: bool = false;
    let mut command_line: bool = false;
    let command_line_command: String;

    let opt = Opt::from_args();
    match RunningAs::decide(opt)
    {
        RunningAs::Interactive(val) =>
        {
            _interactive = val;
            command_line_command = String::from("");
        }
        RunningAs::Script(val) =>
        {
            script = val;
            command_line_command = String::from("");
        }
        RunningAs::CommandLine(val) =>
        {
            command_line = true;
            command_line_command = val.clone();
        }
    }

    if _interactive
    {
        // Set the variable that tells what the shell is.
        env::set_var("0", "ihlsh");
    }

    /* Ensure the shell doesn't quit when trying to kill programs
    ruuning within the shell */
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
        libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    }

    // Get the Username
    let user = get_user_by_uid(get_current_uid()).unwrap();

    let mut command_prompt: String;

    // Making prompt and stuff
    command_prompt = String::from(format!("{} > ", user.name().to_string_lossy()));
    match env::var("PS1")
    {
        Ok(x) => command_prompt = x.clone(),
        Err(_) => (),
    }

    loop
    {
        // The main loop

        let input: String;

        if command_line
        {
            // Strings are pointers, the actual data doesn't get auto copied so we clone it.
            input = String::clone(&command_line_command);
        }
        else if script
        {
            //// input = get_cmd(); //placeholder
            return;
        }
        else
        {
            print!("{}", command_prompt);
            stdout().flush().expect("ihls:: Couldn't flush stdout");
            // Actually get command
            input = get_cmd();
        }
        /* Ensure to skip lines that are empty so the program
        doesn't panic. */
        if input.trim() == ""
        {
            continue;
        }

        let commands = cut_commands(input);
        for command in commands
        {
            for mut dependent_command in command
            {
                let mut is_background = false;
                let dependent_command_last: String;
                match dependent_command.last()
                {
                    Some(s) => dependent_command_last = s.clone(),
                    None => dependent_command_last = String::from(""),
                }

                if String::from("&") == dependent_command_last
                {
                    is_background = true;
                    dependent_command.pop();
                }
                match dependent_command[0].as_str()
                {
                    "exit" =>
                    {
                        if dependent_command.len() < 2
                        {
                            exit_program!(0);
                        }
                        else
                        {
                            let exit_code: i32 = match dependent_command[1].parse()
                            {
                                Ok(x) => x,
                                Err(_) => 1,
                            };

                            exit_program!(exit_code);
                        }
                    }
                    "cd" =>
                    {
                        if dependent_command.len() < 2
                        {
                            let mut home: String = String::from("/");
                            match env::var("HOME")
                            {
                                Ok(x) =>
                                {
                                    home = x.clone();
                                }
                                Err(_) => (),
                            }
                            /* Strings allow for reborrowing.
                            Because we explicitly typed 'home' as a String, we can have rust
                            automatically derefernce the correct number of time. */
                            let home: &str = &home.clone();
                            change_directory!(home);
                            continue;
                        }

                        change_directory!(dependent_command[1].as_str());
                    }
                    "exec" =>
                    {
                        if dependent_command.len() < 2
                        {
                            printerror(String::from("No arguments supplied to 'exec'"));
                        }
                        else
                        {
                            let err = builtins::exec::run(&dependent_command[1..]);
                            match err
                            {
                                exec::Error::Errno(x) => printerror(format!("Exec error: {}", x)),
                                exec::Error::BadArgument(_) => (),
                            }
                        }
                    }

                    _ =>
                    {
                        run_cmd(dependent_command, is_background);
                    }
                }
            }
        }
        if command_line
        {
            break;
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "ihlsh")]
struct Opt
{
    /// Command string to run.
    #[structopt(short, long)]
    command: Option<String>,

    /// A Script to run.
    #[structopt(parse(from_str))]
    script: Option<String>,
}

enum RunningAs
{
    Interactive(bool),
    Script(bool),
    CommandLine(String),
}

impl RunningAs
{
    fn decide(opt: Opt) -> RunningAs
    {
        let mut _i_hate_rust: bool = false;

        match opt.command
        {
            Some(x) => return RunningAs::CommandLine(x.clone()),
            None => _i_hate_rust = true,
        }

        match opt.script
        {
            Some(_) => return RunningAs::Script(true),
            None => _i_hate_rust = true,
        }

        RunningAs::Interactive(true)
    }
}

fn run_cmd(command_tok: Vec<String>, background: bool) -> bool
{
    unsafe {
        let mut command_instance = Command::new(command_tok[0].clone());

        if let Ok(mut child) = command_instance
            .args(&command_tok[1..])
            .pre_exec(|| {
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                Result::Ok(())
            })
            .spawn()
        {
            if !background
            {
                let ret: bool = child.wait().expect("Command did not run!").success();
                print!("\r");
                return ret;
            }

            println!("{}:: {} working...", PROGRAM_NAME, child.id());
            return false; // good
        }

        printerror(format!("{}:: Command not found!", PROGRAM_NAME));
        return true; // bad
    }
}

pub fn printerror(string: String)
{
    eprintln!("{}{}{}", Ansi::RED, string, Ansi::COLOR_END);
}

pub fn get_cmd() -> String
{
    let mut command_string: String = String::new();
    stdin().read_line(&mut command_string).unwrap();
    let command: String = String::from(command_string.trim());
    if command == ""
    {
        return command;
    }

    command
}
