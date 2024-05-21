fn main() {
    let input = std::fs::read_to_string("inputs/day1").unwrap();

    let sum = solve(input.trim(), DIGIT_MAPPING.into_iter());
    println!("Part 1: {sum}");

    let sum = solve(input.trim(), WORD_MAPPING.into_iter().chain(DIGIT_MAPPING));
    println!("Part 2: {sum}");
}

fn solve(input: &str, mapping: impl Iterator<Item = (&'static str, i32)> + Clone) -> i32 {
    input
        .lines()
        .map(|line| {
            let a = first_digit(line, mapping.clone());
            let b = last_digit(line, mapping.clone());
            10 * a + b
        })
        .sum::<i32>()
}

const DIGIT_MAPPING: [(&str, i32); 9] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];
const WORD_MAPPING: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn first_digit(s: &str, mapping: impl Iterator<Item = (&'static str, i32)> + Clone) -> i32 {
    for i in 0..s.len() {
        let mut m = mapping.clone(); // reset the iterator to the beginning
        if let Some(v) = m.find_map(|(k, v)| s[i..].starts_with(k).then_some(v)) {
            return v;
        }
    }
    panic!("invalid input: {s:?}");
}

fn last_digit(s: &str, mapping: impl Iterator<Item = (&'static str, i32)> + Clone) -> i32 {
    for i in (0..s.len()).rev() {
        let mut m = mapping.clone(); // reset the iterator to the beginning
        if let Some(v) = m.find_map(|(k, v)| s[i..].starts_with(k).then_some(v)) {
            return v;
        }
    }
    panic!("invalid input: {s:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(solve(input, DIGIT_MAPPING.into_iter()), 142);
    }

    #[test]
    fn part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(
            solve(input, DIGIT_MAPPING.into_iter().chain(WORD_MAPPING)),
            281
        );
    }
}
