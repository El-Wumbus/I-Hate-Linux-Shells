use exec;
pub fn run(command: &[String]) -> exec::Error
{
    let err = exec::Command::new(command[0].clone())
        .args(&command[1..])
        .exec();
    err
}
