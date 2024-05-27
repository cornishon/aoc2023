use aoc2023::read_grid;
use simple_grid::Grid;

fn main() {
    let input = std::fs::read_to_string("inputs/day14").unwrap();
    let grid = read_grid(&input);
    println!("Part 1: {}", load_of(&roll_north(grid.clone())));
    println!("Part 2: {}", load_after(1_000_000_000, grid));
}

fn cycle(grid: Grid<u8>) -> Grid<u8> {
    roll_east(roll_south(roll_west(roll_north(grid))))
}

fn load_after(time: usize, grid: Grid<u8>) -> usize {
    let mut iters = vec![grid];
    let start = loop {
        let g = cycle(iters.last().unwrap().clone());
        if let Some(i) = iters.iter().position(|x| *x == g) {
            break i;
        }
        iters.push(g);
    };
    let period = iters.len() - start;
    load_of(&iters[start + (time - start) % period])
}

fn load_of(grid: &Grid<u8>) -> usize {
    let h = grid.height();
    grid.cells_with_indices_iter()
        .filter_map(|(idx, c)| (*c == b'O').then_some(idx.row()))
        .fold(0, |acc, row| acc + h - row)
}

fn roll_north(mut grid: Grid<u8>) -> Grid<u8> {
    for col in grid.columns() {
        let mut idx = 0;
        for row in grid.rows() {
            match grid[(col, row)] {
                b'O' => {
                    grid[(col, row)] = b'.';
                    grid[(col, idx)] = b'O';
                    idx += 1;
                }
                b'#' => idx = row + 1,
                _ => {}
            }
        }
    }
    grid
}

fn roll_south(mut grid: Grid<u8>) -> Grid<u8> {
    for col in grid.columns() {
        let mut idx = grid.height();
        for row in grid.rows().rev() {
            match grid[(col, row)] {
                b'O' => {
                    idx -= 1;
                    grid[(col, row)] = b'.';
                    grid[(col, idx)] = b'O';
                }
                b'#' => idx = row,
                _ => {}
            }
        }
    }
    grid
}

fn roll_east(mut grid: Grid<u8>) -> Grid<u8> {
    for row in grid.rows() {
        let mut idx = grid.width();
        for col in grid.columns().rev() {
            match grid[(col, row)] {
                b'O' => {
                    idx -= 1;
                    grid[(col, row)] = b'.';
                    grid[(idx, row)] = b'O';
                }
                b'#' => idx = col,
                _ => {}
            }
        }
    }
    grid
}

fn roll_west(mut grid: Grid<u8>) -> Grid<u8> {
    for row in grid.rows() {
        let mut idx = 0;
        for col in grid.columns() {
            match grid[(col, row)] {
                b'O' => {
                    grid[(col, row)] = b'.';
                    grid[(idx, row)] = b'O';
                    idx += 1;
                }
                b'#' => idx = col + 1,
                _ => {}
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use aoc2023::read_grid;

    use super::*;

    #[test]
    fn can_solve_part1() {
        let grid = read_grid(SAMPLE1);
        assert_eq!(load_of(&roll_north(grid)), 136);
    }

    #[test]
    fn can_solve_part2() {
        let grid = read_grid(SAMPLE1);
        assert_eq!(load_after(1_000_000_000, grid), 64);
    }

    const SAMPLE1: &str = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    ";
}
