use aoc2023::read_grid;
use simple_grid::Grid;

fn main() {
    let input = std::fs::read_to_string("inputs/day14").unwrap();
    let mut grid = read_grid(&input);
    grid.rotate_ccw();

    println!("Part 1: {}", solve_part1(&grid));
    println!("Part 2: {}", solve_part2(grid));
}

fn solve_part1(grid: &Grid<u8>) -> usize {
    load(&roll(grid))
}

fn solve_part2(grid: Grid<u8>) -> usize {
    load_after(1000000000, grid)
}

fn cycle(g0: &Grid<u8>) -> Grid<u8> {
    let mut g1 = roll(g0);
    g1.rotate_cw();
    let mut g2 = roll(&g1);
    g2.rotate_cw();
    let mut g3 = roll(&g2);
    g3.rotate_cw();
    let mut g4 = roll(&g3);
    g4.rotate_cw();
    g4
}

fn load_after(time: usize, grid: Grid<u8>) -> usize {
    let mut iters = vec![grid];
    let (start, period) = loop {
        let g = cycle(iters.last().unwrap());
        if let Some(i) = iters.iter().position(|x| *x == g) {
            break (i, iters.len() - i);
        }
        iters.push(g);
    };
    load(&iters[start + (time - start) % period])
}

fn load(grid: &Grid<u8>) -> usize {
    let w = grid.width();
    grid.cells_with_indices_iter()
        .filter_map(|(idx, c)| (*c == b'O').then_some(idx.column()))
        .fold(0, |acc, col| acc + w - col)
}

fn roll(grid: &Grid<u8>) -> Grid<u8> {
    let mut new = grid.clone();

    for row in grid.rows() {
        let mut s = 0;
        for col in grid.columns() {
            match grid[(col, row)] {
                b'O' => {
                    new[(col, row)] = b'.';
                    new[(s, row)] = b'O';
                    s += 1;
                }
                b'#' => {
                    s = col + 1;
                }
                _ => {}
            }
        }
    }

    new
}

#[cfg(test)]
mod tests {
    use aoc2023::read_grid;

    use super::*;

    #[test]
    fn can_solve_part1() {
        let mut grid = read_grid(SAMPLE1);
        grid.rotate_ccw();
        assert_eq!(solve_part1(&grid), 136);
    }

    #[test]
    fn can_solve_part2() {
        let mut grid = read_grid(SAMPLE1);
        grid.rotate_ccw();
        assert_eq!(load_after(1000000000, grid), 64);
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
