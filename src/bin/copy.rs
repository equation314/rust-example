use std::io;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut buf = [0; 64];
    loop {
        let n = io::stdin().read(&mut buf)?;
        io::stdout().write(&buf[..n])?;
    }
}
