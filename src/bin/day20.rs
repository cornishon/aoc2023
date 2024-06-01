use std::{
    cell::Cell,
    collections::{HashMap, VecDeque},
    ops::Not,
};

use aoc2023::lcm;
use petgraph::prelude::*;
use winnow::{
    combinator::{alt, preceded, separated},
    token::take_while,
    PResult, Parser,
};

fn main() {
    let input = std::fs::read_to_string("inputs/day20").unwrap();
    let (graph, id_map) = parse_input(&input).map_err(|e| println!("{e}")).unwrap();
    println!("{}", solve_part1(graph.clone(), &id_map));
    println!("{}", solve_part2(graph, &id_map));
}

/// A Grpah representing the connections between Modules.
/// The edges are storing the value of the pulses sent.
type Graph = petgraph::Graph<Module, Pulse>;

// Glorified bool, for clarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Pulse {
    #[default]
    Low,
    High,
}

impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        match value {
            true => Pulse::High,
            false => Pulse::Low,
        }
    }
}

impl Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(Cell<bool>),
    Conjunction,
    Broadcast,
    Sink,
}

impl Module {
    fn parser<'i>(i: &mut &'i str) -> PResult<(&'i str, Self)> {
        alt((
            "broadcaster".map(|name: &str| (name, Self::Broadcast)),
            preceded('%', take_while(1.., char::is_alphabetic))
                .map(|name: &str| (name, Self::FlipFlop(Default::default()))),
            preceded('&', take_while(1.., char::is_alphabetic))
                .map(|name: &str| (name, Self::Conjunction)),
        ))
        .parse_next(i)
    }
}

fn parse_input(input: &str) -> PResult<(Graph, HashMap<&str, NodeIndex>)> {
    let mut g = Graph::new();
    let mut id_map: HashMap<&str, NodeIndex> = Default::default();
    let mut out_map: HashMap<&str, Vec<&str>> = Default::default();
    for mut ln in input.trim().lines() {
        let (name, module) = Module::parser.parse_next(&mut ln)?;
        let _ = " -> ".parse_next(&mut ln)?;
        let outputs =
            separated(1.., take_while(1.., char::is_alphabetic), ", ").parse_next(&mut ln)?;
        id_map.insert(name, g.add_node(module));
        out_map.insert(name, outputs);
    }
    for (name, outs) in out_map {
        for out in outs {
            let a = id_map[name];
            if let Some(&b) = id_map.get(out) {
                g.add_edge(a, b, Pulse::Low);
            } else {
                let s = g.add_node(Module::Sink);
                id_map.insert(out, s);
                g.add_edge(a, s, Pulse::Low);
            }
        }
    }
    Ok((g, id_map))
}

fn solve_part1(graph: Graph, id_map: &HashMap<&str, NodeIndex>) -> u32 {
    let mut lows = 0;
    let mut highs = 0;
    let mut machine = Machine::new(graph, id_map["broadcaster"], |_, p| match p {
        Pulse::High => highs += 1,
        Pulse::Low => lows += 1,
    });
    for _ in 0..1000 {
        machine.run();
    }
    highs * lows
}

// For each on the inputs into "rx"'s input, find the period
// between the times it produces a High pulse.
// Assuming "rx"'s input is a Conjunction, the first time "rx" will
// receive a Low pulse is the Lowest Common Multiple of those periods.
fn solve_part2(g: Graph, id_map: &HashMap<&str, NodeIndex>) -> usize {
    let sink_id = id_map["rx"];
    let periods: HashMap<NodeIndex, Cell<usize>> = g
        .edges_directed(sink_id, Incoming)
        .flat_map(|e| {
            let src = e.source();
            assert!(
                g[src] == Module::Conjunction,
                "assuming \"rx\"'s input is a conjunction"
            );
            g.edges_directed(src, Incoming)
                .map(|e| (e.source(), Cell::new(0)))
        })
        .collect();
    let counter = Cell::new(0);
    let mut machine = Machine::new(g, id_map["broadcaster"], |src, pulse| {
        if let Some(n) = periods.get(&src) {
            if pulse == Pulse::High && n.get() == 0 {
                n.set(counter.get());
            }
        }
    });
    loop {
        counter.set(counter.get() + 1);
        machine.run();
        if periods.values().all(|v| v.get() != 0) {
            break periods.into_values().map(|v| v.get()).fold(1, lcm);
        }
    }
}

#[derive(Debug, Clone)]
struct Machine<F: FnMut(NodeIndex, Pulse)> {
    queue: VecDeque<(Pulse, NodeIndex)>,
    graph: Graph,
    broadcast_id: NodeIndex,
    callback: F,
}

impl<F: FnMut(NodeIndex, Pulse)> Machine<F> {
    fn new(graph: Graph, broadcast_id: NodeIndex, callback: F) -> Self {
        Self {
            queue: Default::default(),
            graph,
            broadcast_id,
            callback,
        }
    }

    fn run(&mut self) {
        (self.callback)(NodeIndex::end(), Pulse::Low);
        self.broadcast(self.broadcast_id, Pulse::Low);
        while let Some(msg) = self.queue.pop_front() {
            self.handle(msg);
        }
    }

    fn handle(&mut self, (pulse, dst): (Pulse, NodeIndex)) {
        match &self.graph[dst] {
            Module::FlipFlop(state) => {
                if pulse == Pulse::Low {
                    state.set(!state.get());
                    self.broadcast(dst, state.get().into());
                }
            }
            Module::Conjunction => {
                let mut inputs = self.graph.edges_directed(dst, Incoming);
                let pulse = inputs.any(|e| *e.weight() == Pulse::Low).into();
                self.broadcast(dst, pulse);
            }
            Module::Broadcast => {
                self.broadcast(dst, pulse);
            }
            Module::Sink => {}
        }
    }

    fn broadcast(&mut self, src: NodeIndex, pulse: Pulse) {
        let mut edges = self.graph.neighbors(src).detach();
        while let Some((edge, dst)) = edges.next(&self.graph) {
            (self.callback)(src, pulse);
            self.graph[edge] = pulse;
            self.queue.push_back((pulse, dst));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let (graph, id_map) = parse_input(SAMPLE1).map_err(|e| println!("{e}")).unwrap();
        assert_eq!(solve_part1(graph, &id_map), 32000000);
    }

    #[test]
    fn sample2() {
        let (graph, id_map) = parse_input(SAMPLE2).map_err(|e| println!("{e}")).unwrap();
        assert_eq!(solve_part1(graph, &id_map), 11687500);
    }

    const SAMPLE1: &str = "broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a";
    const SAMPLE2: &str = "broadcaster -> a\n%a -> inv, con\n&inv -> b\n%b -> con\n&con -> output";
}
