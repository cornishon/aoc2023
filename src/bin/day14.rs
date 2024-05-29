use aoc2023::read_2d_array;
use ndarray::{prelude::*, Zip};

fn main() {
    let input = std::fs::read_to_string("inputs/day14").unwrap();
    let grid = read_2d_array(&input);
    println!("Part 1: {}", solve_part1(grid.clone()));
    println!("Part 2: {}", solve_part2(grid));
}

fn solve_part1(mut arr: Array2<u8>) -> usize {
    roll(arr.view_mut());
    load_of(arr.view())
}

fn solve_part2(arr: Array2<u8>) -> usize {
    let mut iters = vec![arr];
    let start = loop {
        let g = cycle(iters.last().unwrap().clone());
        if let Some(i) = iters.iter().position(|x| *x == g) {
            break i;
        }
        iters.push(g);
    };
    let period = iters.len() - start;
    load_of(iters[start + (1_000_000_000 - start) % period].view())
}

fn cycle(grid: Array2<u8>) -> Array2<u8> {
    roll_east(roll_south(roll_west(roll_north(grid))))
}

fn load_of(view: ArrayView2<u8>) -> usize {
    let h = view.ncols();
    Zip::indexed(view).fold(
        0,
        |acc, (y, _), &elem| if elem == b'O' { acc + h - y } else { acc },
    )
}

/// If passed an `array.view_mut()` this rolls north.
/// To roll in other direction, reverse one of the axes and/or swap the axes
fn roll(mut view: ArrayViewMut2<u8>) {
    for x in 0..view.ncols() {
        let mut i = 0;
        for y in 0..view.nrows() {
            match view[[y, x]] {
                b'O' => {
                    view.swap([y, x], [i, x]);
                    i += 1;
                }
                b'#' => i = y + 1,
                _ => {}
            }
        }
    }
}

#[inline]
fn roll_north(mut arr: Array2<u8>) -> Array2<u8> {
    roll(arr.slice_mut(s![.., ..]));
    arr
}

#[inline]
fn roll_south(mut arr: Array2<u8>) -> Array2<u8> {
    roll(arr.slice_mut(s![..;-1, ..]));
    arr
}

#[inline]
fn roll_east(mut arr: Array2<u8>) -> Array2<u8> {
    roll(arr.slice_mut(s![.., ..;-1]).reversed_axes());
    arr
}

#[inline]
fn roll_west(mut arr: Array2<u8>) -> Array2<u8> {
    roll(arr.slice_mut(s![.., ..]).reversed_axes());
    arr
}

#[cfg(test)]
mod tests {
    use aoc2023::read_2d_array;

    use super::*;

    #[test]
    fn can_solve_part1() {
        let grid = read_2d_array(SAMPLE1);
        assert_eq!(solve_part1(grid), 136);
    }

    #[test]
    fn can_solve_part2() {
        let grid = read_2d_array(SAMPLE1);
        assert_eq!(solve_part2(grid), 64);
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
