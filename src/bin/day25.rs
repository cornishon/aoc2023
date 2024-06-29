use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use petgraph::{algo, prelude::*};
use rand::prelude::*;
use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("inputs/day25").unwrap();
    let g = parse_graph(&input);
    let answer = solve(g);
    println!("Answer: {answer}");
}

fn solve(mut g: Graph<(), (&str, &str), Undirected>) -> usize {
    let counter = AtomicUsize::new(0);
    let cut = rayon::iter::repeat(g.clone())
        .map(find_cut)
        .find_any(|cut| {
            counter.fetch_add(1, Ordering::Relaxed);
            cut.len() == 3
        })
        .unwrap();
    eprintln!("retried {} times", counter.into_inner());

    for w in cut.iter() {
        let edge = g
            .edge_references()
            .find_map(|e| (e.weight() == w).then_some(e.id()))
            .unwrap();
        g.remove_edge(edge);
    }
    algo::condensation(g, false)
        .node_weights()
        .map(|w| w.len())
        .product::<usize>()
}

type WireGraph<'a> = Graph<(), (&'a str, &'a str), Undirected>;

fn parse_graph(s: &str) -> WireGraph {
    let mut g = WireGraph::default();
    let mut nodes = HashMap::new();
    for line in s.trim().lines() {
        let (node, children) = line.split_once(':').expect("semicolon");
        let n = *nodes.entry(node).or_insert_with(|| g.add_node(()));
        for child in children.split_whitespace() {
            let c = *nodes.entry(child).or_insert_with(|| g.add_node(()));
            g.add_edge(n, c, (node, child));
        }
    }
    g
}

fn find_cut(mut g: WireGraph) -> Vec<(&str, &str)> {
    let mut rng = rand::thread_rng();
    while g.node_count() > 2 {
        let e = rng.gen_range(0..g.edge_count());
        contract(&mut g, EdgeIndex::new(e));
    }
    let cut = g.edge_weights().copied().collect();
    eprintln!("{cut:?}");
    cut
}

fn contract(g: &mut WireGraph, edge: EdgeIndex) {
    let new_node = g.add_node(());
    let (u, v) = g.edge_endpoints(edge).unwrap();
    for n in [u, v] {
        let mut nbors = g.neighbors(n).detach();
        while let Some((e, w)) = nbors.next(g) {
            if w != v && w != u {
                g.add_edge(new_node, w, g[e]);
            }
        }
    }
    g.remove_node(u);
    g.remove_node(v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve() {
        let answer = solve(parse_graph(SAMPLE1));

        assert_eq!(answer, 54);
    }

    const SAMPLE1: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
}
