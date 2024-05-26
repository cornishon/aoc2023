use std::iter::successors;

use aoc2023::read_grid;
use simple_grid::{Grid, GridIndex};

fn main() {
    let input = std::fs::read_to_string("inputs/day10").unwrap();
    let grid = read_grid(&input, |&b| b);
    println!("Part 1: {}", path(&grid).len() / 2);
    println!("Part 2: {}", enclosed_area(&grid));
}

fn path(grid: &Grid<u8>) -> Vec<GridIndex> {
    successors(Some(path_start(grid)), |(prev, curr)| {
        step(grid, *prev, *curr)
    })
    .map(|(x, _)| x)
    .collect::<Vec<_>>()
}

fn path_start(grid: &Grid<u8>) -> (GridIndex, GridIndex) {
    let start = grid.position(|&c| c == b'S').unwrap();
    let next = [
        start
            .up()
            .and_then(|i| b"|F7".contains(&grid[i]).then_some(i)),
        start
            .right()
            .and_then(|i| b"-7J".contains(&grid[i]).then_some(i)),
        start
            .down()
            .and_then(|i| b"JL|".contains(&grid[i]).then_some(i)),
        start
            .left()
            .and_then(|i| b"FL-".contains(&grid[i]).then_some(i)),
    ]
    .iter()
    .find_map(|opt| *opt)
    .unwrap();
    (start, next)
}

// Based on: https://en.wikipedia.org/wiki/Shoelace_formula
fn enclosed_area(grid: &Grid<u8>) -> i32 {
    let p = path(grid);
    let n = p.len();
    let xs: Vec<_> = p.iter().map(|i| i.column() as i32).collect();
    let ys: Vec<_> = p.iter().map(|i| i.row() as i32).collect();
    let products = (0..n).map(|i| xs[i] * (ys[(i + 1) % n] - ys[(i + n) % n]));
    // because pipes are logically in-between cells, we need to correct the area by (n/2)-1
    products.sum::<i32>().abs() - (n as i32 / 2) + 1
}

fn step(grid: &Grid<u8>, prev: GridIndex, curr: GridIndex) -> Option<(GridIndex, GridIndex)> {
    match grid[curr] {
        b'S' => None,
        b'-' => Some((
            curr,
            if prev.column() < curr.column() { curr.right()? } else { curr.left()? },
        )),
        b'|' => Some((
            curr,
            if prev.row() < curr.row() { curr.down()? } else { curr.up()? },
        )),
        b'L' => Some((
            curr,
            if prev.row() != curr.row() { curr.right()? } else { curr.up()? },
        )),
        b'J' => Some((
            curr,
            if prev.row() != curr.row() { curr.left()? } else { curr.up()? },
        )),
        b'7' => Some((
            curr,
            if prev.row() != curr.row() { curr.left()? } else { curr.down()? },
        )),
        b'F' => Some((
            curr,
            if prev.row() != curr.row() { curr.right()? } else { curr.down()? },
        )),
        _ => panic!("should not leave path"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        assert_eq!(path(&read_grid(SAMPLE1, |&b| b)).len() / 2, 4);
        assert_eq!(path(&read_grid(SAMPLE2, |&b| b)).len() / 2, 4);
        assert_eq!(path(&read_grid(SAMPLE3, |&b| b)).len() / 2, 8);
    }
    #[test]
    fn can_solve_part2() {
        assert_eq!(enclosed_area(&read_grid(SAMPLE4, |&b| b)), 4);
        assert_eq!(enclosed_area(&read_grid(SAMPLE5, |&b| b)), 8);
        assert_eq!(enclosed_area(&read_grid(SAMPLE6, |&b| b)), 10);
    }

    const SAMPLE1: &str = "
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    ";
    const SAMPLE2: &str = "
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    ";
    const SAMPLE3: &str = "
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    ";
    const SAMPLE4: &str = "
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    ";
    const SAMPLE5: &str = "
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    ";
    const SAMPLE6: &str = "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    ";
}
