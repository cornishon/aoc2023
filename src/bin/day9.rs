use std::iter::successors;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("inputs/day9").unwrap();
    println!("Part 1: {}", solve_part1(input.trim()));
    println!("Part 2: {}", solve_part2(input.trim()));
}

fn solve_part1(input: &str) -> i32 {
    input.lines().map(parse_line).map(extrapolate_forward).sum()
}

fn solve_part2(input: &str) -> i32 {
    input.lines().map(parse_line).map(extrapolate_back).sum()
}

fn extrapolate_forward(nums: Vec<i32>) -> i32 {
    successors(Some(nums), |ds| Some(diffs(ds)))
        .map(|v| v.last().copied())
        .while_some()
        .sum()
}

fn extrapolate_back(nums: Vec<i32>) -> i32 {
    successors(Some(nums), |ds| Some(diffs(ds)))
        .map(|v| v.first().copied())
        .while_some()
        .collect_vec() // need a double-ended iterator for rfold
        .into_iter()
        .rfold(0, |acc, x| x - acc)
}

fn diffs(it: &[i32]) -> Vec<i32> {
    it.iter().skip(1).zip(it).map(|(a, b)| a - b).collect()
}

fn parse_line(s: &str) -> Vec<i32> {
    s.split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_extrapolate_forward() {
        assert_eq!(extrapolate_forward(vec![10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(extrapolate_forward(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate_forward(vec![0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn can_extrapolate_backward() {
        assert_eq!(extrapolate_back(vec![10, 13, 16, 21, 30, 45]), 5);
        assert_eq!(extrapolate_back(vec![1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(extrapolate_back(vec![0, 3, 6, 9, 12, 15]), -3);
    }
}
