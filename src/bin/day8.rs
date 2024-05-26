use fxhash::FxHashMap;
use winnow::{
    ascii::{alphanumeric1, newline, space0},
    combinator::{delimited, repeat, separated, separated_pair},
    prelude::*,
    token::any,
};

fn main() {
    let input = std::fs::read_to_string("inputs/day8").unwrap();
    let m = Model::parser
        .parse(input.trim())
        .map_err(|e| {
            println!("{e}");
            std::process::exit(1)
        })
        .unwrap();

    println!("Part 1: {}", solve_part1(&m));
    println!("Part 2: {}", solve_part2(&m));
}

fn solve_part1(m: &Model) -> usize {
    m.path_length("AAA", "ZZZ")
}

fn solve_part2(m: &Model) -> usize {
    m.net
        .keys()
        .filter(|k| k.ends_with('Z'))
        .map(|k| m.path_length(k, k))
        .fold(1, lcm)
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn parser(i: &mut &str) -> PResult<Self> {
        any.verify_map(|c| match c {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None,
        })
        .parse_next(i)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Model {
    dirs: Vec<Dir>,
    net: FxHashMap<String, (String, String)>,
}

impl Model {
    fn path_length(&self, start: &str, end: &str) -> usize {
        let mut i = 0;
        let mut node = &self.net[start];
        let mut dirs = self.dirs.iter().cycle();

        loop {
            let next = match dirs.next().unwrap() {
                Dir::Left => &node.0,
                Dir::Right => &node.1,
            };
            if next == end {
                return i + 1;
            } else {
                node = &self.net[next];
                i += 1;
            }
        }
    }

    fn parser(i: &mut &str) -> PResult<Self> {
        let dirs = repeat(1.., Dir::parser).parse_next(i)?;
        let _ = "\n\n".parse_next(i)?;
        let net = separated(1.., entry_p, newline).parse_next(i)?;
        Ok(Self { dirs, net })
    }
}

fn entry_p(i: &mut &str) -> PResult<(String, (String, String))> {
    separated_pair(
        alphanumeric1.map(str::to_owned),
        (space0, '=', space0),
        delimited(
            '(',
            separated_pair(
                alphanumeric1.map(str::to_owned),
                (space0, ',', space0),
                alphanumeric1.map(str::to_owned),
            ),
            ')',
        ),
    )
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let m = Model::parser.parse(INPUT1).unwrap();
        assert_eq!(solve_part1(&m), 6);
    }

    #[test]
    fn can_solve_part2() {
        let m = Model::parser.parse(INPUT2).unwrap();
        assert_eq!(solve_part2(&m), 6);
    }

    const INPUT1: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    const INPUT2: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
}
