use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

use super::{AdjacencyGraph, DirectedAcyclicGraph, Graph};

impl<V> Graph<V> for DirectedAcyclicGraph<V>
where
    V: Ord + Clone,
{
    fn new() -> Self {
        DirectedAcyclicGraph(AdjacencyGraph::new())
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
        self.0.add_edge(from, to);
    }

    fn remove_node(&mut self, node: &V) {
        self.0.remove_node(node);
    }

    fn remove_edge(&mut self, from: &V, to: &V) {
        self.0.remove_edge(from, to);
    }
}

impl<V> DirectedAcyclicGraph<V>
where
    V: Ord + Eq + Clone + Debug,
{
    pub fn all_paths<F>(&self, start: &V, mut visit_fn: F)
    where
        F: FnMut(Vec<V>) -> bool,
    {
        let mut prev: BTreeMap<V, V> = BTreeMap::new();

        let mut stack: Vec<(V, Option<V>)> = vec![(start.clone(), None)];

        while let Some((node, parent)) = stack.pop() {
            if let Some(p) = parent {
                prev.insert(node.clone(), p.clone());
            }

            let neighbors = self.neighbors(&node);
            if neighbors.is_empty() {
                let mut path = vec![];

                let mut current = node.clone();
                while let Some(next) = prev.get(&current) {
                    path.push(current);
                    current = next.clone();
                }

                path.reverse();

                if !visit_fn(path) {
                    return;
                }
            }

            for n in neighbors {
                stack.push((n.clone(), Some(node.clone())));
            }
        }
    }
}
