use std::process::Command;

fn main() -> std::io::Result<()> {
    let args = ["this", "is", "the", "echo", "command"];
    let mut child = Command::new("./echo").args(&args).spawn()?;
    println!("parent waiting");
    child.wait()?;
    println!("the child exited");
    Ok(())
}
