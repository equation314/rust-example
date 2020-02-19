use std::process::Command;

fn main() -> std::io::Result<()> {
    Command::new("./echo").arg("hello").spawn()?;
    Ok(())
}
