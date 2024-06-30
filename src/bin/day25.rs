use std::collections::HashMap;

use pathfinding::prelude::*;
use petgraph::{algo, prelude::*};

fn main() {
    let input = std::fs::read_to_string("inputs/day25").unwrap();
    let answer = solve(&input);
    println!("Answer: {answer}");
}

fn solve(input: &str) -> usize {
    let g = parse_directed(input);
    let min_cut = find_cut(&g, 3).expect("a cut of size 3 exists");
    let mut g = g.into_edge_type::<Undirected>();
    for edge in min_cut {
        g.remove_edge(edge);
    }
    // Find the connected components and multiply their sizes
    algo::tarjan_scc(&g).into_iter().map(|v| v.len()).product()
}

fn find_cut<N, E>(g: &DiGraph<N, E>, size: usize) -> Option<Vec<EdgeIndex>> {
    let vertices = g.node_indices().collect::<Vec<_>>();
    let sources = g
        .node_indices()
        .filter(|&i| g.edges_directed(i, Incoming).count() == 0)
        .collect::<Vec<_>>();
    let sinks = g
        .node_indices()
        .filter(|&i| g.edges_directed(i, Outgoing).count() == 0)
        .collect::<Vec<_>>();
    // allow flow in both directions
    let caps = g
        .edge_references()
        .flat_map(|e| {
            let s = e.source();
            let t = e.target();
            [((s, t), 1), ((t, s), 1)]
        })
        .collect::<Vec<_>>();

    for source in &sources {
        for sink in &sinks {
            let (_flows, _max_cap, min_cut) =
                edmonds_karp_sparse(&vertices, source, sink, caps.iter().copied());
            if min_cut.len() == size {
                let edges = min_cut
                    .into_iter()
                    .map(|((v, w), _)| g.find_edge_undirected(v, w).unwrap().0)
                    .collect::<Vec<_>>();
                return Some(edges);
            }
        }
    }
    None
}

fn parse_directed(s: &str) -> DiGraph<&str, ()> {
    let mut g = DiGraph::new();
    let mut nodes = HashMap::new();
    for line in s.trim().lines() {
        let (node, children) = line.split_once(':').expect("semicolon");
        let n = *nodes.entry(node).or_insert_with(|| g.add_node(node));
        for child in children.split_whitespace() {
            let c = *nodes.entry(child).or_insert_with(|| g.add_node(child));
            g.add_edge(n, c, ());
        }
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve() {
        let answer = solve(SAMPLE1);
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
