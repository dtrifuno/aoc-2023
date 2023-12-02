use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Game {
    pub id: usize,
    draws: Vec<Draw>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_value: ParseIntError) -> Self {
        ParseError
    }
}

pub fn is_game_possible(game: &Game, red: usize, green: usize, blue: usize) -> bool {
    game.draws
        .iter()
        .all(|d| d.red <= red && d.green <= green && d.blue <= blue)
}

pub fn minimum_power(game: &Game) -> usize {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for draw in &game.draws {
        red = red.max(draw.red);
        green = green.max(draw.green);
        blue = blue.max(draw.blue);
    }

    red * green * blue
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_s = s.split(": ");
        let game_tag = split_s.next().ok_or(ParseError)?;
        let id = game_tag[5..].parse::<usize>()?;

        let draws: Result<Vec<Draw>, ParseError> = split_s
            .next()
            .ok_or(ParseError)?
            .split("; ")
            .map(|s| s.parse())
            .collect();

        Ok(Game { id, draws: draws? })
    }
}

impl FromStr for Draw {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors = ["red", "green", "blue"];
        let mut counts = [0, 0, 0];

        for entry in s.split(", ") {
            for (i, &color) in colors.iter().enumerate() {
                if entry.ends_with(color) {
                    let count = &entry[..(entry.len() - color.len() - 1)].parse::<usize>()?;
                    counts[i] += count;
                }
            }
        }

        Ok(Draw {
            red: counts[0],
            green: counts[1],
            blue: counts[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("17 green, 8 red, 13 blue", Draw { red: 8, green: 17, blue: 13 })]
    #[case("15 blue, 4 red", Draw { red: 4, green:0, blue: 15 })]
    fn test_draw_parse(#[case] s: &str, #[case] expected: Draw) {
        let parsed = s.parse::<Draw>().unwrap();
        assert_eq!(parsed, expected);
    }

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        Game {
            id: 1,
            draws: vec![
                Draw { red: 4, blue: 3, green: 0},
                Draw { red: 1, blue: 6, green: 2},
                Draw { red: 0, blue: 0, green: 2},
            ]
        }
    )]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        Game {
            id: 2,
            draws: vec![
                Draw { red: 0, blue: 1, green: 2},
                Draw { red: 1, blue: 4, green: 3},
                Draw { red: 0, blue: 1, green: 1},
            ]
        }
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        Game {
            id: 3,
            draws: vec![
                Draw { red: 20, blue: 6, green: 8},
                Draw { red: 4, blue: 5, green: 13},
                Draw { red: 1, blue: 0, green: 5},
            ]
        }
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        Game {
            id: 4,
            draws: vec![
                Draw { red: 3, blue: 6, green: 1},
                Draw { red: 6, blue: 0, green: 3},
                Draw { red: 14, blue: 15, green: 3},
            ]
        }
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        Game {
            id: 5,
            draws: vec![
                Draw { red: 6, blue: 1, green: 3},
                Draw { red: 1, blue: 2, green: 2},
            ]
        }
    )]
    fn test_game_parse(#[case] s: &str, #[case] expected: Game) {
        let parsed = s.parse::<Game>().unwrap();
        assert_eq!(parsed, expected);
    }
}
