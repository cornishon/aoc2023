fn main() {
    let input = std::fs::read_to_string("inputs/day4").unwrap();
    println!("Part 1: {}", solve_part1(input.trim()));
    println!("Part 2: {}", solve_part2(input.trim()));
}

fn parse_line(s: &str) -> (Vec<i32>, Vec<i32>) {
    let (_, s) = s.split_once(':').expect(":");
    let (left, right) = s.split_once('|').expect("|");
    let left = left
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().expect("number"))
        .collect();
    let right = right
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().expect("number"))
        .collect();
    (left, right)
}

fn count_matches(winning_numbers: &[i32], given_numbers: &[i32]) -> usize {
    given_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count()
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let (l, r) = parse_line(s);
            1 << count_matches(&l, &r) >> 1
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let scratchpads: Vec<_> = input.trim().lines().map(parse_line).collect();
    let mut counts = vec![1; scratchpads.len()];
    for (i, (l, r)) in scratchpads.iter().enumerate() {
        let num_matches = count_matches(l, r);
        let amount = counts[i];
        for count in &mut counts[i + 1..][..num_matches] {
            *count += amount;
        }
    }
    counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn parse_input() {
        for line in INPUT.lines() {
            let (l, r) = parse_line(line);
            assert!(!l.is_empty());
            assert!(!r.is_empty());
        }
    }

    #[test]
    fn count_wins() {
        let expected = [4, 2, 2, 1, 0, 0];
        for (i, line) in INPUT.lines().enumerate() {
            let (l, r) = parse_line(line);
            assert_eq!(count_matches(&l, &r), expected[i]);
        }
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(INPUT), 13);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(solve_part2(INPUT), 30);
    }
}
