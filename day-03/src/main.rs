use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(())
}
