use std::collections::{BTreeMap, BTreeSet};

use super::{AdjacencyGraph, Graph, UndirectedGraph};

impl<V> Graph<V> for UndirectedGraph<V>
where
    V: Ord + Clone,
{
    fn new() -> Self
    where
        Self: Sized,
    {
        UndirectedGraph {
            directed: AdjacencyGraph::new(),
        }
    }

    fn to_adjecency_graph(&self) -> AdjacencyGraph<V> {
        self.directed.clone()
    }

    fn nodes(&self) -> BTreeSet<V> {
        self.directed.nodes()
    }

    fn adjacencies(&self) -> BTreeMap<V, BTreeSet<V>> {
        self.directed.adjacencies()
    }

    fn neighbors(&self, from: &V) -> BTreeSet<V> {
        self.directed.neighbors(from)
    }

    fn edges(&self) -> BTreeSet<(V, V)> {
        self.directed.edges()
    }

    fn add_node(&mut self, node: V) {
        self.directed.add_node(node);
    }

    fn add_edge(&mut self, from: V, to: V) {
        self.directed.add_edge(from.clone(), to.clone());
        self.directed.add_edge(to, from);
    }

    fn remove_node(&mut self, node: &V) {
        self.directed.remove_node(node);
    }

    fn remove_edge(&mut self, from: &V, to: &V) {
        self.directed.remove_edge(from, to);
        self.directed.remove_edge(to, from);
    }
}
