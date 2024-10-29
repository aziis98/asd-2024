#![allow(dead_code)]

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Debug,
};

use indicatif::{ProgressBar, ProgressIterator};

use super::{AdjacencyGraph, Graph, UndirectedGraph};

impl<V> Graph<V> for AdjacencyGraph<V>
where
    V: Ord + Clone,
{
    fn nodes(&self) -> &BTreeSet<V> {
        &self.nodes
    }

    fn adjacencies(&self) -> &BTreeMap<V, BTreeSet<V>> {
        &self.adjacencies
    }

    fn edges(&self) -> BTreeMap<V, V> {
        self.adjacencies
            .iter()
            .flat_map(|(from, tos)| tos.iter().map(move |to| (from.clone(), to.clone())))
            .collect()
    }
}

impl<V> Graph<V> for UndirectedGraph<V>
where
    V: Ord + Clone,
{
    fn nodes(&self) -> &BTreeSet<V> {
        self.directed.nodes()
    }

    fn adjacencies(&self) -> &BTreeMap<V, BTreeSet<V>> {
        self.directed.adjacencies()
    }

    fn edges(&self) -> BTreeMap<V, V> {
        self.directed.edges()
    }
}

#[allow(dead_code)]
impl<V> AdjacencyGraph<V>
where
    V: Ord + Eq + Clone + Debug,
{
    pub fn new() -> Self {
        AdjacencyGraph {
            nodes: BTreeSet::new(),
            adjacencies: BTreeMap::new(),
        }
    }

    pub fn from_edges(edges: &[(V, V)]) -> Self {
        let mut graph = AdjacencyGraph::new();

        for (from, to) in edges {
            graph.add_edge(from.clone(), to.clone());
        }

        graph
    }

    pub fn add_node(&mut self, node: V) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, from: V, to: V) {
        self.add_node(from.clone());
        self.add_node(to.clone());

        self.adjacencies
            .entry(from)
            .or_insert_with(BTreeSet::new)
            .insert(to);
    }

    pub fn remove_edge(&mut self, from: &V, to: &V) {
        if let Some(adjacencies) = self.adjacencies.get_mut(from) {
            adjacencies.remove(to);
        }
    }

    pub fn get_adjacencies(&self, node: &V) -> Option<&BTreeSet<V>> {
        self.adjacencies.get(node)
    }

    pub fn adjacencies(&self) -> &BTreeMap<V, BTreeSet<V>> {
        &self.adjacencies
    }

    pub fn nodes(&self) -> &BTreeSet<V> {
        &self.nodes
    }

    pub fn edges(&self) -> impl Iterator<Item = (&V, &V)> {
        self.adjacencies
            .iter()
            .flat_map(|(from, tos)| tos.iter().map(move |to| (from, to)))
    }

    pub fn opposite(&self) -> AdjacencyGraph<&V> {
        let mut opposite = AdjacencyGraph::new();

        // O(|E|)
        for (from, to) in self.edges() {
            opposite.add_edge(to, from);
        }

        opposite
    }

    pub fn undirected(&self) -> UndirectedGraph<V> {
        let mut undirected = AdjacencyGraph::new();

        // O(|E|)
        for (from, to) in self.edges() {
            undirected.add_edge(from.clone(), to.clone());
            undirected.add_edge(to.clone(), from.clone());
        }

        UndirectedGraph {
            directed: undirected,
        }
    }

    pub fn restricted(&self, nodes: &Vec<V>) -> AdjacencyGraph<V> {
        let index = nodes.iter().collect::<BTreeSet<_>>();
        let mut restricted = AdjacencyGraph::new();

        for node in nodes {
            restricted.add_node(node.clone());

            if let Some(adjacencies) = self.get_adjacencies(&node) {
                for adj in adjacencies {
                    if index.contains(adj) {
                        restricted.add_edge(node.clone(), adj.clone());
                    }
                }
            }
        }

        restricted
    }

    pub fn has_edge(&self, from: &V, to: &V) -> bool {
        // O(1)
        if let Some(adjacencies) = self.get_adjacencies(from) {
            // O(1)
            adjacencies.contains(&to.to_owned())
        } else {
            false
        }
    }

    pub fn dfs<'a>(&'a self, node: &'a V) -> impl Iterator<Item = V> + 'a {
        let mut visited = BTreeSet::new();
        let mut stack = VecDeque::from([node]);

        std::iter::from_fn(move || {
            while let Some(node) = stack.pop_back() {
                if !visited.insert(node.clone()) {
                    continue;
                }

                if let Some(adjacencies) = self.get_adjacencies(node) {
                    stack.extend(adjacencies);
                }

                return Some(node.clone());
            }

            None
        })
    }

    pub fn shortest_path_matrix(&self) -> BTreeMap<&V, BTreeMap<&V, usize>> {
        let mut result = BTreeMap::new();

        for node in self.nodes.iter() {
            let mut distances = BTreeMap::new();
            let mut visited = BTreeSet::new();
            let mut queue = VecDeque::from([node]);

            distances.insert(node, 0);

            while let Some(node) = queue.pop_front() {
                if visited.contains(node) {
                    continue;
                }

                visited.insert(node.clone());

                let distance = *distances.get(node).unwrap();

                if let Some(adjacencies) = self.get_adjacencies(node) {
                    for adj in adjacencies {
                        if !distances.contains_key(adj) {
                            distances.insert(adj, distance + 1);
                            queue.push_back(adj);
                        }
                    }
                }
            }

            result.insert(node, distances);
        }

        result
    }

    pub fn compute_ccs(&self) -> Vec<Vec<V>> {
        let mut visited = BTreeSet::new();
        let mut result = Vec::new();

        let op = self.opposite();

        for node in self
            .nodes
            .iter()
            .progress()
            .with_style(
                indicatif::ProgressStyle::default_bar()
                    .template("{prefix} {spinner} [{elapsed_precise}] [{wide_bar}] {pos}/{len}")
                    .unwrap(),
            )
            .with_prefix("computing connected components")
        {
            if visited.contains(node) {
                continue;
            }

            let mut cc: BTreeSet<V> = BTreeSet::new();
            let mut stack: Vec<&V> = vec![node];

            while let Some(node) = stack.pop() {
                if cc.contains(node) {
                    continue;
                }

                cc.insert(node.clone());

                if let Some(adjacencies) = self.get_adjacencies(&node) {
                    for adj in adjacencies {
                        stack.push(adj);
                    }
                }

                if let Some(adjacencies) = op.get_adjacencies(&node) {
                    for adj in adjacencies {
                        stack.push(adj);
                    }
                }
            }

            visited.extend(cc.iter().map(|x| x.to_owned()));
            result.push(cc.iter().map(|x| x.to_owned()).collect());
        }

        result
    }

    fn gc(&mut self) {
        let mut to_remove = Vec::new();

        for node in self.nodes.iter() {
            if let Some(adjacencies) = self.get_adjacencies(node) {
                if adjacencies.is_empty() {
                    to_remove.push(node.clone());
                }
            }
        }

        for node in to_remove {
            self.nodes.remove(&node);
            self.adjacencies.remove(&node);
        }
    }
}

impl<V> UndirectedGraph<V>
where
    V: Ord + Eq + Clone + Debug,
{
    pub fn add_edge(&mut self, from: V, to: V) {
        self.directed.add_edge(from.clone(), to.clone());
        self.directed.add_edge(to.clone(), from.clone());
    }

    pub fn remove_edge(&mut self, from: &V, to: &V) {
        self.directed.remove_edge(from, to);
        self.directed.remove_edge(to, from);
    }

    pub fn connected_components(&self) -> Vec<Vec<V>> {
        let mut visited = BTreeSet::new();
        let mut result = Vec::new();

        let pb = ProgressBar::new(self.directed.nodes.len() as u64);

        for node in self.directed.nodes.iter() {
            if visited.contains(node) {
                continue;
            }

            let mut cc: BTreeSet<V> = BTreeSet::new();
            let mut stack: Vec<&V> = vec![node];

            while let Some(node) = stack.pop() {
                if cc.contains(node) {
                    continue;
                }

                pb.inc(1);
                cc.insert(node.clone());

                if let Some(adjacencies) = self.directed.get_adjacencies(&node) {
                    for adj in adjacencies {
                        stack.push(adj);
                    }
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

        let nodes = self.directed.nodes.clone();

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

                while let Some(adjacencies) = self.directed.get_adjacencies(&curr) {
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

                if let Some(adjacencies) = self.directed.get_adjacencies(&curr) {
                    for adj in adjacencies {
                        stack.push(adj.clone());
                    }
                }
            }
        }

        println!("Compacted {} nodes", compacted_count);

        self.directed.gc();

        pb.finish();
    }
}
