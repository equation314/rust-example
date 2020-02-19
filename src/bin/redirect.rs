use std::fs::File;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let file = File::create("output.txt")?;

    let args = ["this", "is", "redirected", "echo"];
    Command::new("./echo")
        .args(&args)
        .stdout(file)
        .spawn()?
        .wait()?;

    Ok(())
}
