use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

use indicatif::ProgressBar;

use super::{AdjacencyGraph, Graph, UndirectedGraph};

impl<V> Graph<V> for UndirectedGraph<V>
where
    V: Ord + Clone,
{
    fn new() -> Self
    where
        Self: Sized,
    {
        UndirectedGraph(AdjacencyGraph::new())
    }

    fn to_adjecency_graph(&self) -> &AdjacencyGraph<V> {
        &self.0
    }

    fn nodes(&self) -> BTreeSet<V> {
        self.0.nodes()
    }

    fn adjacencies(&self) -> BTreeMap<V, BTreeSet<V>> {
        self.0.adjacencies()
    }

    fn neighbors(&self, from: &V) -> BTreeSet<V> {
        self.0.neighbors(from)
    }

    fn edges(&self) -> BTreeSet<(V, V)> {
        self.0.edges()
    }

    fn add_node(&mut self, node: V) {
        self.0.add_node(node);
    }

    fn add_edge(&mut self, from: V, to: V) {
        self.0.add_edge(from.clone(), to.clone());
        self.0.add_edge(to, from);
    }

    fn remove_node(&mut self, node: &V) {
        self.0.remove_node(node);
    }

    fn remove_edge(&mut self, from: &V, to: &V) {
        self.0.remove_edge(from, to);
        self.0.remove_edge(to, from);
    }
}

impl<V> UndirectedGraph<V>
where
    V: Ord + Eq + Clone + Debug,
{
    pub fn add_edge(&mut self, from: V, to: V) {
        self.0.add_edge(from.clone(), to.clone());
        self.0.add_edge(to.clone(), from.clone());
    }

    pub fn remove_edge(&mut self, from: &V, to: &V) {
        self.0.remove_edge(from, to);
        self.0.remove_edge(to, from);
    }

    pub fn connected_components(&self) -> Vec<Vec<V>> {
        let mut visited = BTreeSet::new();
        let mut result = Vec::new();

        let pb = ProgressBar::new(self.0.nodes.len() as u64);

        for node in self.0.nodes.iter() {
            if visited.contains(node) {
                continue;
            }

            let mut cc: BTreeSet<V> = BTreeSet::new();
            let mut stack: Vec<V> = vec![node.clone()];

            while let Some(node) = stack.pop() {
                if cc.contains(&node) {
                    continue;
                }

                pb.inc(1);
                cc.insert(node.clone());

                for adj in self.neighbors(&node) {
                    stack.push(adj);
                }
            }

            visited.extend(cc.iter().map(|x| x.to_owned()));
            result.push(cc.iter().map(|x| x.to_owned()).collect());
        }

        pb.finish();

        result
    }

    // This runs a depth-first search on the graph searching for o--o--o paths and removes the middle node
    // recursively until no more o--o--o paths are found.
    pub fn compact_chains(&mut self) {
        let mut visited = BTreeSet::new();

        let nodes = self.0.nodes.clone();

        let pb = ProgressBar::new(nodes.len() as u64);

        let mut compacted_count = 0;

        for node in nodes {
            if visited.contains(&node) {
                continue;
            }

            let mut stack = vec![node];

            while let Some(node) = stack.pop() {
                if visited.contains(&node) {
                    continue;
                }

                pb.inc(1);
                visited.insert(node.clone());

                // while adj has only one neighbor
                let mut curr = node;
                let mut path = vec![curr.clone()];

                loop {
                    let adjacencies = self.neighbors(&curr);
                    if adjacencies.is_empty() {
                        break;
                    }

                    let probes = adjacencies
                        .iter()
                        .filter(|&x| !path.contains(x))
                        .collect::<Vec<_>>();

                    if probes.len() != 1 {
                        break;
                    }

                    curr = probes[0].clone();

                    visited.insert(curr.clone());
                    path.push(curr.clone());
                }

                if path.len() < 3 {
                    continue;
                }

                path.windows(2).for_each(|x| {
                    self.remove_edge(&x[0], &x[1]);
                });

                self.add_edge(path[0].clone(), path[path.len() - 1].clone());

                compacted_count += path.len() - 2;

                for adj in self.neighbors(&curr) {
                    stack.push(adj);
                }
            }
        }

        println!("Compacted {} nodes", compacted_count);

        self.0.gc();

        pb.finish();
    }
}
