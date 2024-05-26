use std::iter::repeat;

use fxhash::FxHashMap;
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("inputs/day12").unwrap();

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let lines = input.trim().lines().map(parse_line);
    lines.map(count).sum()
}

fn solve_part2(input: &str) -> usize {
    let lines = input.trim().lines().map(parse_line);
    lines.map(|l| count(quintuple(l))).sum()
}

fn parse_line(s: &str) -> Line {
    let (pat, nums) = s.trim().split_once(' ').unwrap();
    let pattern = pat.to_owned();
    let nums = nums.split(',').map(|n| n.parse().unwrap()).collect();
    Line { pattern, nums }
}

struct Line {
    pattern: String,
    nums: Vec<usize>,
}

fn quintuple(line: Line) -> Line {
    let pattern = repeat(line.pattern.as_str()).take(5).join("?");
    let nums = line.nums.repeat(5);
    Line { pattern, nums }
}

fn count(line: Line) -> usize {
    let pattern = line.pattern.trim_end_matches('.').as_bytes();
    count_memo(pattern.to_vec(), line.nums, &mut FxHashMap::default())
}

fn count_memo(
    pattern: Vec<u8>,
    nums: Vec<usize>,
    memo: &mut FxHashMap<(Vec<u8>, Vec<usize>), usize>,
) -> usize {
    if nums.is_empty() {
        return if pattern.contains(&b'#') { 0 } else { 1 };
    }
    if pattern.len() < min_len(&nums) {
        return 0;
    }
    if pattern.starts_with(b".") {
        return count_memo(trim_start(&pattern), nums, memo);
    }
    if pattern.starts_with(b"#") {
        if pattern[..nums[0]].contains(&b'.') || pattern[nums[0]..].starts_with(b"#") {
            return 0;
        }
        return count_memo(
            pattern.get(nums[0] + 1..).unwrap_or_default().to_vec(),
            nums.get(1..).unwrap_or_default().to_vec(),
            memo,
        );
    }

    if let Some(n) = memo.get(&(pattern.clone(), nums.clone())) {
        return *n;
    }
    let value = {
        let mut pattern = pattern.clone();
        pattern[0] = b'#';
        let l = count_memo(pattern[1..].to_vec(), nums.clone(), memo);
        let r = count_memo(pattern, nums.clone(), memo);
        l + r
    };
    memo.insert((pattern, nums), value);
    value
}

#[inline]
fn min_len(nums: &[usize]) -> usize {
    nums.iter().sum::<usize>() + nums.len() - 1
}

#[inline]
fn trim_start(mut pattern: &[u8]) -> Vec<u8> {
    while let [b'.', rest @ ..] = pattern {
        pattern = rest;
    }
    pattern.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern2() {
        let line = Line {
            pattern: "?###????????".to_owned(),
            nums: vec![3, 2, 1],
        };
        assert_eq!(count(line), 10);
    }

    #[test]
    fn pattern1() {
        let line = Line {
            pattern: "???.###".to_owned(),
            nums: vec![1, 1, 3],
        };
        assert_eq!(count(line), 1);
    }

    #[test]
    fn sample1_part1() {
        let sample = "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1";
        assert_eq!(solve_part1(sample), 21);
    }

    #[test]
    fn sample1_part2() {
        let sample = "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1";
        assert_eq!(solve_part2(sample), 525152);
    }
}
