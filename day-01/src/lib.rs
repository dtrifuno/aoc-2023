pub fn calibration_value(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first_digit = digits.next().unwrap();
    let last_digit = digits.last().unwrap_or(first_digit);

    10 * first_digit + last_digit
}

pub fn calibration_value_with_spelling(line: &str) -> u32 {
    let mut digits = (0..line.len()).filter_map(|i| starting_digit(&line[i..]));
    let first_digit = digits.next().unwrap();
    let last_digit = digits.last().unwrap_or(first_digit);

    10 * first_digit + last_digit
}

fn starting_digit(s: &str) -> Option<u32> {
    let spelled_digits = [
        ("one", 1u32),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    // Check if string starts with a digit char
    let c = s.chars().next()?;
    if let r @ Some(_) = c.to_digit(10) {
        return r;
    }

    // Otherwise, check if string starts with a spelled out digit
    for (from, to) in spelled_digits {
        if s.starts_with(from) {
            return Some(to);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    #[case("six3fqrvmrcrspsix7ptsseight", 37)]
    fn test_calibration_value(#[case] line: &str, #[case] expected_value: u32) {
        let value = calibration_value(line);
        assert_eq!(value, expected_value);
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("six3fqrvmrcrspsix7ptsseight", 68)]
    #[case("2pqpqgppm63ccptb", 23)]
    #[case("rtdsxdz53seveneightsixzbtrbbm", 56)]
    fn value_with_spelling(#[case] line: &str, #[case] expected_value: u32) {
        let value = calibration_value_with_spelling(line);
        assert_eq!(value, expected_value);
    }
}
