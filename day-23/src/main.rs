use aoc::{petgraph::prelude::*, FxHashMap, FxHashSet};

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let graph = make_graph(input);
    let mut sets = FxHashSet::default();

    for t in graph.node_indices() {
        if !graph[t].starts_with('t') {
            continue;
        }

        for u in graph.neighbors(t) {
            for v in graph.neighbors(t) {
                if graph.contains_edge(u, v) {
                    let mut set = [t, u, v];
                    set.sort();
                    sets.insert(set);
                }
            }
        }
    }

    sets.len() as u64
}

fn two(input: &str) -> Vec<String> {
    let graph = make_graph(input);

    let mut cliques = Vec::new();
    let nodes = graph.node_indices().collect();
    maximal_clique(
        &graph,
        &mut cliques,
        &mut Vec::new(),
        nodes,
        FxHashSet::default(),
    );

    let mut party = cliques
        .into_iter()
        .max_by_key(|r| r.len())
        .unwrap()
        .into_iter()
        .map(|nx| graph[nx].to_string())
        .collect::<Vec<_>>();
    party.sort();
    party
}

// adapted from https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn maximal_clique(
    graph: &UnGraph<&str, ()>,
    cliques: &mut Vec<Vec<NodeIndex>>,
    r: &mut Vec<NodeIndex>,
    mut p: FxHashSet<NodeIndex>,
    mut x: FxHashSet<NodeIndex>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    let pivot = p
        .union(&x)
        .copied()
        .max_by_key(|u| graph.neighbors(*u).count())
        .unwrap();
    let pivot_neighbors = graph.neighbors(pivot).collect();

    for v in p.clone().difference(&pivot_neighbors).copied() {
        r.push(v);
        let v_neighbors = graph.neighbors(v).collect();
        let p_new = p.intersection(&v_neighbors).copied().collect();
        let x_new = x.intersection(&v_neighbors).copied().collect();
        maximal_clique(graph, cliques, r, p_new, x_new);
        r.pop();

        p.remove(&v);
        x.insert(v);
    }
}

fn make_graph(input: &str) -> UnGraph<&str, ()> {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = FxHashMap::default();

    for line in input.lines() {
        let u = &line[0..2];
        let v = &line[3..5];

        let u = *nodes.entry(u).or_insert_with(|| graph.add_node(u));
        let v = *nodes.entry(v).or_insert_with(|| graph.add_node(v));
        graph.add_edge(u, v, ());
    }

    graph
}
