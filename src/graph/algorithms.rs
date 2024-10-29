#![allow(dead_code)]

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Debug,
};

use indicatif::ProgressIterator;

use super::{AdjacencyGraph, Graph, UndirectedGraph};

#[allow(dead_code)]
impl<V> AdjacencyGraph<V>
where
    V: Ord + Eq + Clone + Debug,
{
    pub fn opposite(&self) -> AdjacencyGraph<V> {
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
        let nodes_index = nodes.iter().collect::<BTreeSet<_>>();
        let mut restricted = AdjacencyGraph::new();

        for node in nodes {
            for adj in self.neighbors(&node) {
                if nodes_index.contains(&adj) {
                    restricted.add_edge(node.clone(), adj.clone());
                }
            }
        }

        restricted
    }

    pub fn has_edge(&self, from: &V, to: &V) -> bool {
        self.neighbors(from).contains(to)
    }

    pub fn shortest_path_matrix(&self) -> BTreeMap<V, BTreeMap<V, usize>> {
        let mut result = BTreeMap::new();

        for node in self.nodes.iter() {
            let mut distances = BTreeMap::new();
            let mut visited = BTreeSet::new();
            let mut queue = VecDeque::from([node.clone()]);

            distances.insert(node.clone(), 0);

            while let Some(node) = queue.pop_front() {
                if visited.contains(&node) {
                    continue;
                }

                visited.insert(node.clone());

                let distance = *distances.get(&node).unwrap();

                for adj in self.neighbors(&node) {
                    if !distances.contains_key(&adj) {
                        distances.insert(adj.clone(), distance + 1);
                        queue.push_back(adj.clone());
                    }
                }
            }

            result.insert(node.clone(), distances);
        }

        result
    }

    pub fn compute_ccs(&self) -> Vec<Vec<V>> {
        let mut visited = BTreeSet::new();
        let mut result = Vec::new();

        let op = self.opposite();

        println!("Computing connected components...");

        for node in self.nodes.iter().progress() {
            if visited.contains(node) {
                continue;
            }

            let mut cc: BTreeSet<V> = BTreeSet::new();
            let mut stack: Vec<V> = vec![node.clone()];

            while let Some(node) = stack.pop() {
                if cc.contains(&node) {
                    continue;
                }

                cc.insert(node.clone());

                for adj in self.neighbors(&node) {
                    stack.push(adj);
                }

                for adj in op.neighbors(&node) {
                    stack.push(adj);
                }
            }

            visited.extend(cc.iter().map(|x| x.to_owned()));
            result.push(cc.iter().map(|x| x.to_owned()).collect());
        }

        result
    }

    pub fn gc(&mut self) {
        let mut to_remove = Vec::new();

        for node in self.nodes.iter() {
            if self.neighbors(node).is_empty() {
                to_remove.push(node.clone());
            }
        }

        for node in to_remove {
            self.nodes.remove(&node);
            self.adjacencies.remove(&node);
        }
    }
}
