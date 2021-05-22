use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f = File::create("foo.txt")?;
    f.write_all(b"Hello, world!\n")?;
    f.write_all("Goodbyé\n".as_bytes())?;
    f.sync_data()?;
    Ok(())
}
