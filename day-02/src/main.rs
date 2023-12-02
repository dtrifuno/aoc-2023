use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use day_02::{is_game_possible, minimum_power, Game};

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let games = contents
        .split('\n')
        .map(|s| s.parse::<Game>().unwrap())
        .collect::<Vec<_>>();

    let mut valid_ids_sum = 0;
    for game in &games {
        if is_game_possible(game, 12, 13, 14) {
            valid_ids_sum += game.id;
        }
    }
    println!("{valid_ids_sum}");

    let total_power: usize = games.iter().map(minimum_power).sum();
    println!("{total_power}");

    Ok(())
}
