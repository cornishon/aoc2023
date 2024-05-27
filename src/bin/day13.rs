use std::iter::zip;

use aoc2023::read_grid;
use simple_grid::Grid;

fn main() {
    let input = std::fs::read_to_string("inputs/day13").unwrap();
    let grids: Vec<_> = input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(read_grid)
        .collect();
    println!("Part1: {}", solve_part1(&grids));
    println!("Part2: {}", solve_part2(&grids));
}

fn solve_part1(grids: &[Grid<u8>]) -> usize {
    let hs: usize = grids.iter().filter_map(horizontal_reflection).sum();
    let vs: usize = grids.iter().filter_map(vertical_reflection).sum();
    vs + 100 * hs
}

fn solve_part2(grids: &[Grid<u8>]) -> usize {
    let hs: usize = grids.iter().filter_map(horizontal_smudge).sum();
    let vs: usize = grids.iter().filter_map(vertical_smudge).sum();
    vs + 100 * hs
}

fn find1(size: usize, predicate: impl FnMut(Vec<(usize, usize)>) -> bool) -> Option<usize> {
    pairs(size).position(predicate).map(|i| i + 1)
}

fn pairs(limit: usize) -> impl Iterator<Item = Vec<(usize, usize)>> {
    (0..limit - 1).map(move |n| {
        (0..=n)
            .map(|i| (n - i, n + 1 + i))
            .take_while(|(_, y)| *y < limit)
            .collect()
    })
}

fn vertical_reflection(grid: &Grid<u8>) -> Option<usize> {
    find1(grid.width(), |ps| {
        ps.iter()
            .all(|&(n, m)| grid.column_iter(n).eq(grid.column_iter(m)))
    })
}

fn horizontal_reflection(grid: &Grid<u8>) -> Option<usize> {
    find1(grid.height(), |ps| {
        ps.iter()
            .all(|&(n, m)| grid.row_iter(n).eq(grid.row_iter(m)))
    })
}

fn vertical_smudge(grid: &Grid<u8>) -> Option<usize> {
    find1(grid.width(), |ps| {
        let defects = ps.iter().map(|&(n, m)| {
            zip(grid.column_iter(n), grid.column_iter(m))
                .filter(|(a, b)| a != b)
                .count()
        });
        defects.sum::<usize>() == 1
    })
}

fn horizontal_smudge(grid: &Grid<u8>) -> Option<usize> {
    find1(grid.height(), |ps| {
        let defects = ps.iter().map(|&(n, m)| {
            zip(grid.row_iter(n), grid.row_iter(m))
                .filter(|(a, b)| a != b)
                .count()
        });
        defects.sum::<usize>() == 1
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid1_part1() {
        let g = read_grid(GRID1);
        assert_eq!(vertical_reflection(&g), Some(5));
        assert_eq!(horizontal_reflection(&g), None);
    }

    #[test]
    fn grid2_part1() {
        let g = read_grid(GRID2);
        assert_eq!(vertical_reflection(&g), None);
        assert_eq!(horizontal_reflection(&g), Some(4));
    }

    #[test]
    fn grid1_part2() {
        let g = read_grid(GRID1);
        assert_eq!(horizontal_smudge(&g), Some(3));
        assert_eq!(vertical_smudge(&g), None);
    }

    #[test]
    fn grid2_part2() {
        let g = read_grid(GRID2);
        assert_eq!(horizontal_smudge(&g), Some(1));
        assert_eq!(vertical_smudge(&g), None);
    }

    const GRID1: &str = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.";

    const GRID2: &str = "
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";
}
