use std::env;
use std::path::Path;

#[macro_export]
macro_rules! change_directory {
    ($path:expr) => {
        if builtins::cd::run($path)
        {
            printerror(format!(
                "{}:: Failed to change the directory!",
                PROGRAM_NAME
            ));
        }
    };
}

pub fn run(strpath: &str) -> bool
{
    let mut cdpath = Path::new(strpath);
    let home: &Path = Path::new("/");
    if cdpath == Path::new("")
    {
        cdpath = Path::new(&home);
    }
    match env::set_current_dir(&cdpath)
    {
        Err(_) =>
        {
            return true;
        }
        _ => (),
    }
    return false;
}
