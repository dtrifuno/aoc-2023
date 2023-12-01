use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use day_01::{calibration_value, calibration_value_with_spelling};

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let first_sum: u32 = contents.trim().split('\n').map(calibration_value).sum();
    println!("Sum of calibration values when ignoring spelled out digits is {first_sum}.");

    let second_sum: u32 = contents
        .trim()
        .split('\n')
        .map(calibration_value_with_spelling)
        .sum();
    println!("Sum of calibration values including spelled out digits is {second_sum}.");
    Ok(())
}
