use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Debug,
};

use indicatif::ProgressIterator;

use super::{AdjacencyGraph, Graph, UndirectedGraph};

impl<V> Graph<V> for AdjacencyGraph<V>
where
    V: Ord + Clone,
{
    fn new() -> Self
    where
        Self: Sized,
    {
        AdjacencyGraph {
            nodes: BTreeSet::new(),
            adjacencies: BTreeMap::new(),
        }
    }

    fn to_adjecency_graph(&self) -> &AdjacencyGraph<V> {
        &self
    }

    fn nodes(&self) -> BTreeSet<V> {
        self.nodes.clone()
    }

    fn adjacencies(&self) -> BTreeMap<V, BTreeSet<V>> {
        self.adjacencies.clone()
    }

    fn edges(&self) -> BTreeSet<(V, V)> {
        self.adjacencies
            .iter()
            .flat_map(|(from, tos)| tos.iter().map(move |to| (from.clone(), to.clone())))
            .collect()
    }

    fn neighbors(&self, from: &V) -> BTreeSet<V> {
        if let Some(neighbors) = self.adjacencies.get(from) {
            neighbors.clone()
        } else {
            BTreeSet::new()
        }
    }

    fn add_node(&mut self, node: V) {
        self.nodes.insert(node);
    }

    fn add_edge(&mut self, from: V, to: V) {
        self.nodes.insert(from.clone());
        self.nodes.insert(to.clone());

        self.adjacencies
            .entry(from)
            .or_insert_with(BTreeSet::new)
            .insert(to.clone());
    }

    fn remove_node(&mut self, node: &V) {
        self.nodes.remove(node);
        self.adjacencies.remove(node);

        for adjacencies in self.adjacencies.values_mut() {
            adjacencies.remove(node);
        }
    }

    fn remove_edge(&mut self, from: &V, to: &V) {
        if let Some(adjacencies) = self.adjacencies.get_mut(from) {
            adjacencies.remove(to);
        }
    }
}

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

        UndirectedGraph(undirected)
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
