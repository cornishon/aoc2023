use std::collections::VecDeque;

use aoc2023::read_grid;
use fxhash::FxHashMap;
use simple_grid::{Grid, GridIndex};

fn main() {
    let input = std::fs::read_to_string("inputs/day21").unwrap();
    let (start, grid) = parse_input(&input);
    println!("Part 1: {}", solve_part1(&grid, start, 64));
    let extra_copies = 1; // 1 is enough for the big grid, while 3 is needed for the example grid
                          // TODO: figure out how to calculate the heuristic based on input
    println!(
        "Part 2: {}",
        solve_part2(&grid, start, 26501365, extra_copies)
    );
}

fn parse_input(input: &str) -> (GridIndex, Grid<bool>) {
    let grid = read_grid(input);
    let start = grid.position(|&x| x == b'S').unwrap();
    let (w, h) = grid.dimensions();
    let grid = Grid::new(w, h, grid.into_iter().map(|b| b != b'#').collect());
    (start, grid)
}

fn distances(grid: &Grid<bool>, start: GridIndex, n: i16) -> FxHashMap<Coord, usize> {
    let w = grid.width() as u16;
    let h = grid.height() as u16;
    assert_eq!(w, h, "expected a square grid");
    let mut queue = VecDeque::from([(start.into(), 0)]);
    let mut dict = FxHashMap::default();
    while let Some((c @ Coord { t_row, t_col, x, y }, d)) = queue.pop_front() {
        if dict.contains_key(&c)
            || !grid[(x as usize, y as usize)]
            || t_row.abs() > n
            || t_col.abs() > n
        {
            continue;
        }
        for n in c.neighbors(w, h) {
            queue.push_back((n, d + 1));
        }
        dict.insert(c, d);
    }
    dict
}

fn solve_part1(grid: &Grid<bool>, start: GridIndex, limit: usize) -> usize {
    distances(grid, start, 0)
        .into_values()
        .filter(|&v| v % 2 == 0 && v <= limit)
        .count()
}

// The infinite grid with base tile RxC can be represented as
//   ^^^^^
//   |||||
// <-CEEEC->
// <-E...E->
// <-E...E->
// <-E...E->
// <-CEEEC->
//   |||||
//   vvvvv
// where:
// The `.`s are bruteforced.
// The edges `E` represent all the the points in the infinite grid as represented by the arrows next to them.
//   Can add arbitrarily many R to distance
// The corners `C` represent all the the points in the infinite grid as represented and enclosed by the arrows next to them.
//   Represents everything in that quadrant. can add arbitrarily many R or C to that distance
//
// Reference:
// https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/21.py
fn solve_part2(grid: &Grid<bool>, start: GridIndex, limit: usize, n: i16) -> usize {
    let ds = distances(grid, start, n);
    let w = grid.width() as u16;
    let h = grid.height() as u16;
    let mut cache = FxHashMap::default();
    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {
            if !ds.contains_key(&Coord::new(0, 0, x, y)) {
                continue;
            }
            for t_row in -n..=n {
                for t_col in -n..=n {
                    let d = ds[&Coord::new(t_row, t_col, x, y)];
                    if d <= limit && (d & 1 == limit & 1) {
                        ans += 1;
                    }
                    if t_row.abs() == n && t_col.abs() == n {
                        // corner
                        ans += solve(&mut cache, d, true, h, limit);
                    } else if t_row.abs() == n || t_col.abs() == n {
                        // edge
                        ans += solve(&mut cache, d, false, h, limit);
                    }
                }
            }
        }
    }
    ans
}

fn solve(
    cache: &mut FxHashMap<(usize, bool), usize>,
    steps: usize,
    corner: bool,
    n_rows: u16,
    limit: usize,
) -> usize {
    *cache.entry((steps, corner)).or_insert_with(|| {
        let n = n_rows as usize;
        let amount = limit.saturating_sub(steps) / n;
        let mut ret_val = 0;
        for x in 1..amount + 1 {
            let d = steps + n * x;
            if d <= limit && (d & 1 == limit & 1) {
                ret_val += (corner as usize * x) + 1;
            }
        }
        ret_val
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: u16,
    y: u16,
    t_row: i16,
    t_col: i16,
}

impl From<GridIndex> for Coord {
    fn from(idx: GridIndex) -> Self {
        Self {
            x: idx.column() as u16,
            y: idx.row() as u16,
            t_row: 0,
            t_col: 0,
        }
    }
}

impl Coord {
    fn new(t_row: i16, t_col: i16, x: u16, y: u16) -> Self {
        Self { t_row, t_col, x, y }
    }
    fn neighbors(self, w: u16, h: u16) -> [Self; 4] {
        [self.left(w), self.right(w), self.up(h), self.down(h)]
    }
    fn left(self, w: u16) -> Self {
        let Coord { t_row, t_col, x, y } = self;
        if x == 0 {
            Coord::new(t_row - 1, t_col, w - 1, y)
        } else {
            Coord::new(t_row, t_col, x - 1, y)
        }
    }
    fn right(self, w: u16) -> Self {
        let Coord { t_row, t_col, x, y } = self;
        if x == w - 1 {
            Coord::new(t_row + 1, t_col, 0, y)
        } else {
            Coord::new(t_row, t_col, x + 1, y)
        }
    }
    fn up(self, h: u16) -> Self {
        let Coord { t_row, t_col, x, y } = self;
        if y == 0 {
            Coord::new(t_row, t_col - 1, x, h - 1)
        } else {
            Coord::new(t_row, t_col, x, y - 1)
        }
    }
    fn down(self, h: u16) -> Self {
        let Coord { t_row, t_col, x, y } = self;
        if y == h - 1 {
            Coord::new(t_row, t_col + 1, x, 0)
        } else {
            Coord::new(t_row, t_col, x, y + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let (start, grid) = parse_input(SAMPLE1);
        assert_eq!(solve_part1(&grid, start, 6), 16);
    }

    #[test]
    fn can_solve_part2() {
        let (start, grid) = parse_input(SAMPLE1);
        const N: i16 = 3;
        assert_eq!(solve_part2(&grid, start, 6, N), 16);
        assert_eq!(solve_part2(&grid, start, 10, N), 50);
        assert_eq!(solve_part2(&grid, start, 50, N), 1594);
        assert_eq!(solve_part2(&grid, start, 100, N), 6536);
        assert_eq!(solve_part2(&grid, start, 500, N), 167004);
        assert_eq!(solve_part2(&grid, start, 1000, N), 668697);
        assert_eq!(solve_part2(&grid, start, 5000, N), 16733044);
    }

    const SAMPLE1: &str = "
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    ";
}
