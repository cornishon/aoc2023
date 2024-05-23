fn main() {
    let input = std::fs::read_to_string("inputs/day6").unwrap();
    println!("Part 1: {}", solve_part1(input.trim()));
    println!("Part 2: {}", solve_part2(input.trim()));
}

#[inline]
fn distance(time: usize, race_duration: usize) -> usize {
    time * (race_duration - time)
}

fn solve_part1(input: &str) -> usize {
    let (line1, line2) = input.split_once('\n').unwrap();
    let durations = parse_many("Time:", line1);
    let records = parse_many("Distance:", line2);
    assert_eq!(durations.len(), records.len());

    std::iter::zip(durations, records)
        .map(|(duration, record)| {
            (0..duration)
                .map(|t| distance(t, duration))
                .filter(|&d| d > record)
                .count()
        })
        .product()
}

fn solve_part2(input: &str) -> usize {
    let (line1, line2) = input.split_once('\n').unwrap();
    let duration = parse_single("Time:", line1);
    let record = parse_single("Distance:", line2);
    (0..duration)
        .map(|t| distance(t, duration))
        .filter(|&d| d > record)
        .count()
}

fn parse_many(prefix: &str, line: &str) -> Vec<usize> {
    line.strip_prefix(prefix)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_single(prefix: &str, line: &str) -> usize {
    line.strip_prefix(prefix)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        assert_eq!(solve_part1(INPUT), 288);
    }

    #[test]
    fn can_solve_part2() {
        assert_eq!(solve_part2(INPUT), 71503);
    }

    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";
}
