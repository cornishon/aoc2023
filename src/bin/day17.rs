use aoc2023::read_grid_with;
use pathfinding::prelude::*;
use simple_grid::Grid;

fn main() {
    let input = std::fs::read_to_string("inputs/day17").unwrap();
    let grid = read_grid_with(&input, |&c| c - b'0');

    println!("Part 1: {}", find_path(&grid, 1, 3));
    println!("Part 2: {}", find_path(&grid, 4, 10));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Heading {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum TurnDir {
    Left,
    Straight,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    steps: usize,
    pos: (usize, usize),
    heading: Heading,
}

impl Heading {
    fn turn(self, dir: TurnDir) -> Self {
        match dir {
            TurnDir::Left => match self {
                Heading::Right => Heading::Up,
                Heading::Left => Heading::Down,
                Heading::Up => Heading::Left,
                Heading::Down => Heading::Right,
            },
            TurnDir::Right => match self {
                Heading::Right => Heading::Down,
                Heading::Left => Heading::Up,
                Heading::Up => Heading::Right,
                Heading::Down => Heading::Left,
            },
            TurnDir::Straight => self,
        }
    }
}

fn advance((x, y): (usize, usize), delta: usize, dir: Heading) -> (usize, usize) {
    match dir {
        Heading::Right => (x.wrapping_add(delta), y),
        Heading::Left => (x.wrapping_sub(delta), y),
        Heading::Up => (x, y.wrapping_sub(delta)),
        Heading::Down => (x, y.wrapping_add(delta)),
    }
}

fn find_path(m: &Grid<u8>, min_steps: usize, max_steps: usize) -> usize {
    let start = State {
        pos: (0, 0),
        heading: Heading::Right,
        steps: 0,
    };
    let (width, height) = m.dimensions();
    let end = (width - 1, height - 1);

    let cost = |(xa, ya): (usize, usize), (xb, yb): (usize, usize)| {
        if ya == yb {
            (xa.min(xb)..=xa.max(xb))
                .map(|x| m[(x, ya)] as usize)
                .sum::<usize>()
                - m[(xa, ya)] as usize
        } else {
            debug_assert_eq!(xa, xb, "non-straight path");
            (ya.min(yb)..=ya.max(yb))
                .map(|y| m[(xa, y)] as usize)
                .sum::<usize>()
                - m[(xa, ya)] as usize
        }
    };

    let step = |state: State, d| {
        let heading = state.heading.turn(d);
        let steps = if d == TurnDir::Straight { state.steps + 1 } else { min_steps };
        let delta = if d == TurnDir::Straight { 1 } else { min_steps };
        State {
            pos: advance(state.pos, delta, heading),
            heading,
            steps,
        }
    };

    const DIRS: [TurnDir; 3] = [TurnDir::Left, TurnDir::Straight, TurnDir::Right];
    let successors = |state: State| {
        DIRS.into_iter()
            .map(move |d| step(state, d))
            .filter(|s| s.steps <= max_steps && s.pos.0 < width && s.pos.1 < height)
            .map(move |s| (s, cost(state.pos, s.pos)))
    };

    dijkstra(&start, |s| successors(*s), |s| s.pos == end)
        .expect("path always exists")
        .1
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE1: &str = "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";

    const SAMPLE2: &str = "
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991";

    #[test]
    fn sample1_part1() {
        let m = read_grid_with(SAMPLE1, |&c| c - b'0');
        assert_eq!(find_path(&m, 1, 3), 102)
    }

    #[test]
    fn sample1_part2() {
        let m = read_grid_with(SAMPLE1, |&c| c - b'0');
        assert_eq!(find_path(&m, 4, 10), 94)
    }

    #[test]
    fn sample2_part2() {
        let m = read_grid_with(SAMPLE2, |&c| c - b'0');
        assert_eq!(find_path(&m, 4, 10), 71)
    }
}
