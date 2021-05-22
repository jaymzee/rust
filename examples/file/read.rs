use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut contents = String::new();
    {
        let mut file = File::open("read.rs")?;
        file.read_to_string(&mut contents)?;
    }
    println!("{}", contents);
    Ok(())
}
