use std::fs;

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        println!("{}", entry?.file_name().to_str().unwrap());
    }
    Ok(())
}
