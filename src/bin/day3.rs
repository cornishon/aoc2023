use aoc2023::read_grid_with;
use simple_grid::{Grid, GridIndex};

fn main() {
    let input = std::fs::read_to_string("inputs/day3").unwrap();
    println!("Part1: {}", solve_part1(&input));
    println!("Part2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> u32 {
    let grid = read_grid_with(input, |b| *b as char);
    let (w, h) = grid.dimensions();
    let mut visited = Grid::new(w, h, vec![false; w * h]);
    grid.cells_with_indices_iter()
        .filter_map(|(i, c)| (c != &'.' && c.is_ascii_punctuation()).then_some(i))
        .map(|symbol| {
            let mut sum: u32 = 0;
            for nbor in grid.neighbor_indices_of(symbol) {
                if visited[nbor] || !grid[nbor].is_ascii_digit() {
                    continue;
                }
                sum += number_at(nbor, &grid, &mut visited);
            }
            sum
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let grid = read_grid_with(input, |b| *b as char);
    let (w, h) = grid.dimensions();
    let mut visited = Grid::new(w, h, vec![false; w * h]);
    grid.cells_with_indices_iter()
        .filter_map(|(i, &c)| (c == '*').then_some(i))
        .map(|gear| {
            let mut nums = Vec::new();
            for nbor in grid.neighbor_indices_of(gear) {
                if visited[nbor] || !grid[nbor].is_ascii_digit() {
                    continue;
                }
                nums.push(number_at(nbor, &grid, &mut visited));
            }
            if nums.len() == 2 {
                nums.into_iter().product::<u32>()
            } else {
                0
            }
        })
        .sum()
}

fn number_at(nbor: GridIndex, grid: &Grid<char>, visited: &mut Grid<bool>) -> u32 {
    visited[nbor] = true;
    let mut start = nbor;
    let mut end = nbor;
    while let Some(i) = grid.left_index(start).and_then(|i| {
        visited[i] = true;
        grid[i].is_ascii_digit().then_some(i)
    }) {
        start = i;
    }
    while let Some(i) = grid.right_index(end).and_then(|i| {
        visited[i] = true;
        grid[i].is_ascii_digit().then_some(i)
    }) {
        end = i;
    }
    let (col, row) = (start.column(), start.row());
    (0..=end.column() - start.column()).fold(0, |num, i| {
        10 * num + grid[(col + i, row)].to_digit(10).unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn can_parse_input() {
        let grid = read_grid_with(INPUT, |b| *b as char);
        println!("{}", grid.to_pretty_string());
    }

    #[test]
    fn can_solve_part1() {
        assert_eq!(solve_part1(INPUT), 4361);
    }

    #[test]
    fn can_solve_part2() {
        assert_eq!(solve_part2(INPUT), 467835);
    }
}
