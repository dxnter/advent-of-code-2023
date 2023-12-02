advent_of_code::solution!(1);

/// Calculates the calibration value from the given line of text
///
/// Determined by concatenating the first and last integers in
/// the string into a two digit number. When the line contains
/// one only digit, it's doubled to form a two digit number.
fn calculate_calibration_value(line: &str) -> Option<u32> {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    match (digits.next(), digits.last()) {
        (Some(first), Some(last)) => Some(first * 10 + last),
        (Some(single), None) => Some(single * 10 + single),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .filter_map(calculate_calibration_value)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    // Map spelled out numbers to a string containing the integer value
    // The integer is surrounded by the prefix and suffix character to
    // handle overlapping spelled out numbers
    static INT_MAP: &[(&str, &str)] = &[
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    input
        .lines()
        .map(|line| {
            INT_MAP
                .iter()
                .fold(line.to_string(), |acc, &(word, digit)| {
                    acc.replace(word, digit)
                })
                .to_string()
        })
        .filter_map(|line| calculate_calibration_value(&line))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
