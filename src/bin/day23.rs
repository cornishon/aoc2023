use std::collections::VecDeque;

use aoc2023::read_matrix;
use pathfinding::prelude::{directions, Matrix};
use petgraph::{algo, prelude::*};
use smallvec::SmallVec;

fn main() {
    let input = std::fs::read_to_string("inputs/day23").unwrap();
    let (m, start, end) = parse_input(&input);
    println!("Part 1: {}", solve_part1(&m, start, end));
    println!("Part 2: {}", solve_part2(&m, start, end));

    // println!(
    //     "{:?}",
    //     petgraph::dot::Dot::with_config(&g.graph, &[/* petgraph::dot::Config::EdgeNoLabel */])
    // );
}

fn parse_input(input: &str) -> (Matrix<u8>, (usize, usize), (usize, usize)) {
    let m = read_matrix(input);
    let start = m.as_ref()[..m.columns]
        .iter()
        .position(|val| *val == b'.')
        .map(|c| (0, c))
        .expect("have start");
    let end = m.as_ref()[(m.rows - 1) * m.columns..]
        .iter()
        .rposition(|val| *val == b'.')
        .map(|c| (m.rows - 1, c))
        .expect("have end");
    (m, start, end)
}

fn solve_part1(m: &Matrix<u8>, start: (usize, usize), end: (usize, usize)) -> usize {
    let chart = parse_graph(m, start, true);
    solve(&chart, start, end)
}

fn solve_part2(m: &Matrix<u8>, start: (usize, usize), end: (usize, usize)) -> usize {
    let chart = parse_graph(m, start, false);
    solve(&chart, start, end)
}

fn solve(chart: &Chart, start: (usize, usize), end: (usize, usize)) -> usize {
    algo::all_simple_paths(&chart, start, end, 0, None)
        .map(|p: Vec<_>| p.windows(2).map(|e| chart[(e[0], e[1])]).sum::<usize>())
        .max()
        .unwrap()
}

type Chart = DiGraphMap<(usize, usize), usize>;

fn parse_graph(m: &Matrix<u8>, start: (usize, usize), with_slopes: bool) -> Chart {
    let mut graph = DiGraphMap::new();
    let nbors = if with_slopes { neighbours_with_slopes } else { neighbours_without_slopes };
    let next = m.move_in_direction(start, directions::S).unwrap();
    let mut queue = VecDeque::from([(graph.add_node(start), next)]);
    while let Some((node, mut curr)) = queue.pop_front() {
        let mut prev = node;
        let mut w = 0;
        loop {
            match nbors(prev, curr, m).as_slice() {
                &[n] => {
                    prev = curr;
                    curr = n;
                    w += 1;
                }
                ns => {
                    if !graph.contains_edge(node, curr) {
                        graph.add_node(curr);
                        graph.add_edge(node, curr, w + 1);
                        for n in ns {
                            queue.push_back((curr, *n));
                        }
                    }
                    break;
                }
            }
        }
    }
    graph
}

fn neighbours_with_slopes(
    prev: (usize, usize),
    idx: (usize, usize),
    m: &Matrix<u8>,
) -> SmallVec<[(usize, usize); 3]> {
    let dir = match m[idx] {
        b'v' => directions::S,
        b'^' => directions::N,
        b'<' => directions::W,
        b'>' => directions::E,
        _ => {
            return m
                .neighbours(idx, false)
                .filter(|&i| i != prev && m[i] != b'#')
                .collect()
        }
    };
    m.move_in_direction(idx, dir)
        .into_iter()
        .filter(|&i| i != prev && m[i] != b'#')
        .collect()
}

fn neighbours_without_slopes(
    prev: (usize, usize),
    idx: (usize, usize),
    m: &Matrix<u8>,
) -> SmallVec<[(usize, usize); 3]> {
    m.neighbours(idx, false)
        .filter(|&i| i != prev && m[i] != b'#')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let (m, start, end) = parse_input(SAMPLE1);
        assert_eq!(solve_part1(&m, start, end), 94);
    }

    #[test]
    fn can_solve_part2() {
        let (m, start, end) = parse_input(SAMPLE1);
        assert_eq!(solve_part2(&m, start, end), 154);
    }

    const SAMPLE1: &str = "
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    ";
}
