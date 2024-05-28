use aoc2023::read_grid;
use rayon::prelude::*;
use simple_grid::Grid;

fn main() {
    let input = std::fs::read_to_string("inputs/day16").unwrap();
    let m = read_grid(&input);
    let starting_beam = Beam {
        dir: Dir::Right,
        pos: (0, 0),
    };
    println!("Part 1: {}", simulate(&m, starting_beam));
    println!("Part 2: {}", find_most_energized(&m));
}

fn find_most_energized(grid: &Grid<u8>) -> usize {
    let (w, h) = grid.dimensions();
    let starting_beams = (grid.columns().map(|x| Beam {
        dir: Dir::Down,
        pos: (x, 0),
    }))
    .chain(grid.columns().map(|x| Beam {
        dir: Dir::Up,
        pos: (x, h - 1),
    }))
    .chain(grid.rows().map(|y| Beam {
        dir: Dir::Right,
        pos: (0, y),
    }))
    .chain(grid.rows().map(|y| Beam {
        dir: Dir::Left,
        pos: (w - 1, y),
    }));
    starting_beams
        .par_bridge()
        .map(|beam| simulate(grid, beam))
        .max()
        .expect("iterator is not empty")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Left = 0b0001,
    Up = 0b0010,
    Right = 0b0100,
    Down = 0b1000,
}

#[inline]
fn step(stack: &mut Vec<Beam>, dirs: &Grid<u8>, beam: Beam) {
    let b = beam.advance();
    if dirs.get(b.pos).is_some_and(|d| d & b.dir as u8 == 0) {
        stack.push(b);
    }
}

fn simulate(grid: &Grid<u8>, starting_beam: Beam) -> usize {
    let mut stack = vec![starting_beam];
    let mut dirs = Grid::new_default(grid.width(), grid.height());
    while let Some(beam) = stack.pop() {
        dirs[beam.pos] |= beam.dir as u8;
        match grid[beam.pos] {
            b'.' => step(&mut stack, &dirs, beam),
            b'-' => match beam.dir {
                Dir::Left | Dir::Right => step(&mut stack, &dirs, beam),
                _ => {
                    step(&mut stack, &dirs, beam.turn(Dir::Left));
                    step(&mut stack, &dirs, beam.turn(Dir::Right));
                }
            },
            b'|' => match beam.dir {
                Dir::Up | Dir::Down => step(&mut stack, &dirs, beam),
                _ => {
                    step(&mut stack, &dirs, beam.turn(Dir::Up));
                    step(&mut stack, &dirs, beam.turn(Dir::Down));
                }
            },
            b'/' => match beam.dir {
                Dir::Left => step(&mut stack, &dirs, beam.turn(Dir::Down)),
                Dir::Up => step(&mut stack, &dirs, beam.turn(Dir::Right)),
                Dir::Right => step(&mut stack, &dirs, beam.turn(Dir::Up)),
                Dir::Down => step(&mut stack, &dirs, beam.turn(Dir::Left)),
            },
            b'\\' => match beam.dir {
                Dir::Left => step(&mut stack, &dirs, beam.turn(Dir::Up)),
                Dir::Up => step(&mut stack, &dirs, beam.turn(Dir::Left)),
                Dir::Right => step(&mut stack, &dirs, beam.turn(Dir::Down)),
                Dir::Down => step(&mut stack, &dirs, beam.turn(Dir::Right)),
            },
            t => panic!("unexpected tile: {t}"),
        }
    }
    dirs.into_iter().filter(|d| *d != 0).count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    dir: Dir,
    pos: (usize, usize),
}

impl Beam {
    #[inline]
    fn turn(self, dir: Dir) -> Self {
        Self { dir, ..self }
    }

    #[inline]
    fn advance(mut self) -> Self {
        let Self { dir, pos: (x, y) } = self;
        match dir {
            Dir::Left => self.pos = (x.wrapping_sub(1), y),
            Dir::Up => self.pos = (x, y.wrapping_sub(1)),
            Dir::Right => self.pos = (x.wrapping_add(1), y),
            Dir::Down => self.pos = (x, y.wrapping_add(1)),
        }
        self
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE1: &str = r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....";

    #[test]
    fn part1() {
        let m = read_grid(SAMPLE1);
        let n = simulate(
            &m,
            crate::Beam {
                dir: crate::Dir::Right,
                pos: (0, 0),
            },
        );
        assert_eq!(n, 46);
    }
    #[test]
    fn part2() {
        let m = read_grid(SAMPLE1);
        let n = find_most_energized(&m);
        assert_eq!(n, 51);
    }
}
