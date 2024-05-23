use itertools::Itertools;
use std::ops::Range;

fn main() {
    let input = std::fs::read_to_string("inputs/day5").unwrap();
    println!("Part 1: {}", solve_part1(input.trim()));
    println!("Part 2: {}", solve_part2(input.trim()));
}

#[allow(clippy::single_range_in_vec_init)]
fn solve_part2(input: &str) -> i64 {
    let (header, rest) = input.split_once("\n\n").expect("seeds");
    let maps = rest.split("\n\n").map(parse_map).collect_vec();
    parse_seeds(header)
        .tuples()
        .flat_map(|(start, len)| {
            maps.iter().fold(vec![start..start + len], |rs, map| {
                rs.iter()
                    .flat_map(|r| map.transform_range(r.clone()))
                    .collect_vec()
            })
        })
        .filter(|r| !r.is_empty())
        .map(|r| r.start)
        .min()
        .unwrap()
}

fn solve_part1(input: &str) -> i64 {
    let (header, rest) = input.split_once("\n\n").expect("seeds");
    let maps = rest.split("\n\n").map(parse_map).collect_vec();
    parse_seeds(header)
        .map(|seed| maps.iter().fold(seed, |s, map| map.transform(s)))
        .min()
        .unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mapping {
    range: Range<i64>,
    offset: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    label: String,
    mappings: Vec<Mapping>,
}

impl Map {
    fn transform(&self, i: i64) -> i64 {
        match find_map_item(&self.mappings, i) {
            Some(m) => i + m.offset,
            None => i,
        }
    }

    fn transform_range(&self, r: Range<i64>) -> Vec<Range<i64>> {
        let ms = &self.mappings;
        let mut start = r.start;

        let Some(a) = search_sorted_mappings(ms, start) else {
            return vec![r];
        };

        let mut result = Vec::new();
        for m in &ms[a..] {
            if start < m.range.start {
                if r.end <= m.range.start {
                    result.push(start..r.end);
                    return result;
                }
                result.push(start..m.range.start);
                start = m.range.start;
            }
            let end = m.range.end.min(r.end);
            result.push(start + m.offset..end + m.offset);
            start = end;
            if start >= r.end {
                break;
            }
        }
        if start <= r.end {
            result.push(start..r.end);
        }

        result
    }
}

fn find_map_item(items: &[Mapping], x: i64) -> Option<Mapping> {
    search_sorted_mappings(items, x).and_then(|i| {
        if items[i].range.contains(&x) {
            Some(items[i].clone())
        } else {
            None
        }
    })
}

fn parse_seeds(seeds: &str) -> impl Iterator<Item = i64> + '_ {
    let (_, seeds) = seeds.split_once(':').expect("seed:");
    seeds
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().expect("number"))
}

fn parse_mapping(s: &str) -> Mapping {
    let (dst, start, len) = s
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().expect("number"))
        .collect_tuple()
        .expect("exactly 3 numbers");
    Mapping {
        range: start..start + len,
        offset: dst - start,
    }
}

fn parse_map(s: &str) -> Map {
    let (lbl, map) = s.trim().split_once('\n').expect("header");
    let label = lbl.strip_suffix(" map:").unwrap().to_owned();
    let mut mappings = map.lines().map(parse_mapping).collect_vec();
    mappings.sort_unstable_by_key(|m| m.range.start);
    Map { label, mappings }
}

fn search_sorted_mappings(mappings: &[Mapping], x: i64) -> Option<usize> {
    let mut a = 0;
    let mut b = mappings.len() - 1;

    if x < mappings[a].range.start || x >= mappings[b].range.end {
        return None;
    }

    while b - a > 1 {
        let mid = (a + b) / 2;
        if x < mappings[mid].range.start {
            b = mid;
            continue;
        }
        if x >= mappings[mid].range.end {
            a = mid;
            continue;
        }
        return Some(mid);
    }

    if x >= mappings[a].range.end {
        Some(b)
    } else {
        Some(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_search_sorted_ranges() {
        let ranges = [0..3, 5..7, 10..20].map(|r| Mapping {
            range: r,
            offset: 0,
        });
        assert_eq!(search_sorted_mappings(&ranges, -1), None);
        assert_eq!(search_sorted_mappings(&ranges, 1), Some(0));
        assert_eq!(search_sorted_mappings(&ranges, 7), Some(2));
        assert_eq!(search_sorted_mappings(&ranges, 10), Some(2));
        assert_eq!(search_sorted_mappings(&ranges, 20), None);
        assert_eq!(search_sorted_mappings(&ranges, 21), None);
    }

    #[test]
    fn can_parse_input() {
        let (header, rest) = INPUT.split_once("\n\n").unwrap();
        let seeds = parse_seeds(header).collect_vec();
        let maps = rest.split("\n\n").map(parse_map).collect_vec();
        println!("{seeds:?}");
        println!("{maps:?}");
    }

    #[test]
    fn can_solve_part1() {
        assert_eq!(solve_part1(INPUT), 35);
    }

    #[test]
    fn can_solve_part2() {
        assert_eq!(solve_part2(INPUT), 46);
    }

    const INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}
