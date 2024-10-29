use std::collections::{BTreeMap, BTreeSet};

use super::{AdjacencyGraph, Graph};

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

    fn to_adjecency_graph(&self) -> AdjacencyGraph<V> {
        self.clone()
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
