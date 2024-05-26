use std::collections::BTreeSet;

use aoc2023::read_grid;
use simple_grid::Grid;

fn main() {
    let input = std::fs::read_to_string("inputs/day11").unwrap();
    let grid = read_grid(&input, |b| *b);
    println!("Part 1: {}", solve(&grid, 2));
    println!("Part 2: {}", solve(&grid, 1_000_000));
}

fn solve(grid: &Grid<u8>, factor: usize) -> usize {
    let frows = free_rows(grid);
    let fcols = free_columns(grid);

    let mut sum = 0;
    for a @ (ax, ay) in galaxies(grid) {
        for b @ (bx, by) in galaxies(grid) {
            if a < b {
                let dy = distance(ay, by, &frows, factor);
                let dx = distance(ax, bx, &fcols, factor);
                sum += dx + dy;
            }
        }
    }
    sum
}

fn galaxies(grid: &Grid<u8>) -> impl Iterator<Item = (usize, usize)> + '_ {
    grid.cells_with_indices_iter()
        .filter_map(|(i, x)| (*x == b'#').then_some((i.column(), i.row())))
}

fn free_rows(grid: &Grid<u8>) -> BTreeSet<usize> {
    grid.rows()
        .filter(|&i| grid.row_iter(i).all(|x| *x != b'#'))
        .collect()
}

fn free_columns(grid: &Grid<u8>) -> BTreeSet<usize> {
    grid.columns()
        .filter(|&i| grid.column_iter(i).all(|x| *x != b'#'))
        .collect()
}

fn distance(x1: usize, x2: usize, free_space: &BTreeSet<usize>, factor: usize) -> usize {
    x1.abs_diff(x2) + free_space.range(x1.min(x2)..x1.max(x2)).count() * (factor - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let grid = read_grid(SAMPLE1, |b| *b);
        assert_eq!(solve(&grid, 2), 374)
    }

    #[test]
    fn can_solve_part2() {
        let grid = read_grid(SAMPLE1, |b| *b);
        assert_eq!(solve(&grid, 10), 1030);
        assert_eq!(solve(&grid, 100), 8410);
    }

    const SAMPLE1: &str = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    ";
}
