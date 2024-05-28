use std::iter::successors;

use winnow::{
    ascii::{dec_int, hex_uint, space1},
    combinator::{alt, delimited, seq},
    PResult, Parser,
};

fn main() {
    let input = std::fs::read_to_string("inputs/day18").unwrap();
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> i64 {
    enclosed_area(&dig_path(input.trim().lines().map(|l| {
        Entry::parser.parse(l).map_err(|e| println!("{e}")).unwrap()
    })))
}

fn solve_part2(input: &str) -> i64 {
    enclosed_area(&dig_path(input.trim().lines().map(|l| {
        let e = Entry::parser.parse(l).map_err(|e| println!("{e}")).unwrap();
        Entry {
            dir: Dir::from_u32(e.color % 16),
            amount: e.color as i64 / 16,
            ..e
        }
    })))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Dir {
    fn parser(i: &mut &str) -> PResult<Self> {
        alt((
            'L'.value(Dir::Left),
            'R'.value(Dir::Right),
            'U'.value(Dir::Up),
            'D'.value(Dir::Down),
        ))
        .parse_next(i)
    }

    fn from_u32(x: u32) -> Self {
        match x {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("not a direction: {x}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Entry {
    dir: Dir,
    amount: i64,
    color: u32,
}

impl Entry {
    fn parser(i: &mut &str) -> PResult<Self> {
        seq!(Entry {
            dir: Dir::parser,
            amount: delimited(space1, dec_int, space1),
            color: delimited("(#", hex_uint, ")")
        })
        .parse_next(i)
    }
}

fn dig_path(mut entires: impl Iterator<Item = Entry>) -> Vec<(i64, i64)> {
    successors(Some((0, 0)), move |prev| {
        entires.next().map(|e| advance(*prev, e.amount, e.dir))
    })
    .collect()
}

// Based on: https://en.wikipedia.org/wiki/Shoelace_formula
fn enclosed_area(p: &[(i64, i64)]) -> i64 {
    let n = p.len();
    let product = (0..n)
        .map(|i| p[i].0 * (p[(i + 1) % n].1 - p[(i + n) % n].1))
        .sum::<i64>();
    let perimeter = p
        .iter()
        .zip(&p[1..])
        .map(|((x0, y0), (x1, y1))| (x0 - *x1).abs() + (y0 - *y1).abs())
        .sum::<i64>();
    product.abs() + (perimeter / 2) + 1
}

#[inline]
fn advance((x, y): (i64, i64), delta: i64, dir: Dir) -> (i64, i64) {
    match dir {
        Dir::Left => (x.wrapping_sub(delta), y),
        Dir::Right => (x.wrapping_add(delta), y),
        Dir::Up => (x, y.wrapping_sub(delta)),
        Dir::Down => (x, y.wrapping_add(delta)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        assert_eq!(solve_part1(SAMPLE1), 62);
    }

    #[test]
    fn can_solve_part2() {
        assert_eq!(solve_part2(SAMPLE1), 952408144115);
    }

    const SAMPLE1: &str = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
    ";
}
