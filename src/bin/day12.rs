use std::{hash::BuildHasherDefault, iter::repeat};

use fxhash::FxHasher;
use itertools::Itertools;

type FxHashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<FxHasher>>;

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

fn count(mut line: Line) -> usize {
    let mut pattern = trim_end(line.pattern.as_bytes().to_vec());
    // reverse the vectors since we'd otherwise be popping from the front
    pattern.reverse();
    line.nums.reverse();
    count_memo(pattern, line.nums, &mut FxHashMap::default())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct View<'a> {
    pattern: &'a [u8],
    nums: &'a [usize],
}

impl<'a> View<'a> {
    fn new(pattern: &'a [u8], nums: &'a [usize]) -> Self {
        Self { pattern, nums }
    }
}

impl hashbrown::Equivalent<(Vec<u8>, Vec<usize>)> for View<'_> {
    fn equivalent(&self, key: &(Vec<u8>, Vec<usize>)) -> bool {
        self.pattern == key.0 && self.nums == key.1
    }
}

fn count_memo(
    mut pattern: Vec<u8>,
    mut nums: Vec<usize>,
    memo: &mut FxHashMap<(Vec<u8>, Vec<usize>), usize>,
) -> usize {
    if nums.is_empty() {
        return if pattern.contains(&b'#') { 0 } else { 1 };
    }
    if pattern.len() < min_len(&nums) {
        return 0;
    }
    if let Some(b'.') = pattern.last() {
        return count_memo(trim_end(pattern), nums, memo);
    }
    if let Some(b'#') = pattern.last() {
        let n = nums.pop().expect("non-empty: checked above");
        let i = pattern.len().wrapping_sub(n + 1); // index from the end
        if pattern.get(i) == Some(&b'#') || pattern[i.wrapping_add(1)..].contains(&b'.') {
            return 0;
        }
        pattern.truncate(pattern.len().saturating_sub(n + 1));
        return count_memo(pattern, nums, memo);
    }
    if let Some(n) = memo.get(&View::new(&pattern, &nums)) {
        return *n;
    }

    let value = {
        let mut pat = pattern.clone();
        pat.pop();
        let l = count_memo(pat.clone(), nums.clone(), memo);
        pat.push(b'#');
        let r = count_memo(pat, nums.clone(), memo);
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
fn trim_end(mut pattern: Vec<u8>) -> Vec<u8> {
    while let Some(b'.') = pattern.last() {
        pattern.pop();
    }
    pattern
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
