#[macro_export]
macro_rules! exit_program {
    ($code:expr) => {
        builtins::exit::run($code);
    };
}

pub fn run(code: i32) { std::process::exit(code); }
