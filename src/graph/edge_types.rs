#![allow(dead_code)]

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    hash::Hash,
};

use indicatif::ProgressBar;

use super::{AdjacencyGraph, DirectedAcyclicGraph, Graph};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum EdgeType {
    TreeEdge,
    BackEdge,
    ForwardEdge,
    CrossEdge,
}

struct ClassifyState<V> {
    progress_bar: ProgressBar,

    edge_types: BTreeMap<(V, V), EdgeType>,

    visited: BTreeSet<V>,

    start_times: BTreeMap<V, i32>,
    finished_nodes: BTreeSet<V>,

    time: i32,
}

impl<V> ClassifyState<V>
where
    V: Ord + Eq + Clone + Debug,
{
    pub fn classify_edges_rec(mut self, graph: &AdjacencyGraph<V>) -> BTreeMap<(V, V), EdgeType> {
        for start in graph.nodes().iter() {
            if self.visited.contains(start) {
                continue;
            }

            self.dfs(graph, start, None);
        }

        self.progress_bar.finish();
        return self.edge_types;
    }

    pub fn dfs(&mut self, graph: &AdjacencyGraph<V>, node: &V, parent: Option<&V>) {
        if self.visited.contains(node) {
            return;
        }

        self.progress_bar.inc(1);
        self.visited.insert(node.clone());
        self.time += 1;
        self.start_times.insert(node.clone(), self.time);

        if let Some(parent) = parent {
            self.edge_types
                .insert((parent.clone(), node.clone()), EdgeType::TreeEdge);
        }

        for adj in graph.neighbors(node) {
            if !self.visited.contains(&adj) {
                self.dfs(graph, &adj, Some(node));
            } else {
                if !self.finished_nodes.contains(&adj) {
                    self.edge_types
                        .insert((node.clone(), adj.clone()), EdgeType::BackEdge);
                } else if self.start_times.get(node) < self.start_times.get(&adj) {
                    self.edge_types
                        .insert((node.clone(), adj.clone()), EdgeType::ForwardEdge);
                } else {
                    self.edge_types
                        .insert((node.clone(), adj.clone()), EdgeType::CrossEdge);
                }
            }
        }

        self.time += 1;
        self.finished_nodes.insert(node.clone());
    }
}

impl<V> AdjacencyGraph<V>
where
    V: Ord + Eq + Clone + Debug,
{
    pub fn compute_edge_types_rec(&self) -> BTreeMap<(V, V), EdgeType> {
        return ClassifyState {
            progress_bar: ProgressBar::new(self.nodes().len() as u64),

            edge_types: BTreeMap::new(),
            visited: BTreeSet::new(),
            start_times: BTreeMap::new(),
            finished_nodes: BTreeSet::new(),
            time: 0,
        }
        .classify_edges_rec(self);
    }

    pub fn compute_edge_types(&self) -> BTreeMap<(V, V), EdgeType> {
        let mut edge_types = BTreeMap::new();
        let mut visited = BTreeSet::new();
        let mut start_times = BTreeMap::new();
        let mut finished_nodes = BTreeSet::new();
        let mut time = 0;

        let progress_bar = ProgressBar::new(self.nodes().len() as u64);

        enum Continuation<V> {
            Start { node: V, parent: Option<V> },
            Neighbors { node: V, continue_from: usize },
            End { node: V },
        }

        for start in self.nodes().iter() {
            if visited.contains(start) {
                continue;
            }

            let mut continuations = vec![Continuation::Start {
                node: start.clone(),
                parent: None,
            }];

            while let Some(continuation) = continuations.pop() {
                match continuation {
                    Continuation::Start { node, parent } => {
                        continuations.push(Continuation::End { node: node.clone() });

                        progress_bar.inc(1);
                        visited.insert(node.clone());
                        time += 1;
                        start_times.insert(node.clone(), time);

                        if let Some(parent) = parent {
                            edge_types.insert((parent.clone(), node.clone()), EdgeType::TreeEdge);
                        }

                        continuations.push(Continuation::Neighbors {
                            node: node.clone(),
                            continue_from: 0,
                        });
                    }
                    Continuation::Neighbors {
                        node,
                        continue_from: index,
                    } => {
                        for (i, adj) in self.neighbors(&node).iter().enumerate() {
                            if i < index {
                                continue;
                            }

                            if !visited.contains(adj) {
                                continuations.push(Continuation::Neighbors {
                                    node: node.clone(),
                                    continue_from: i + 1,
                                });
                                continuations.push(Continuation::Start {
                                    node: adj.clone(),
                                    parent: Some(node.clone()),
                                });
                                break;
                            } else {
                                if !finished_nodes.contains(adj) {
                                    edge_types
                                        .insert((node.clone(), adj.clone()), EdgeType::BackEdge);
                                } else if start_times.get(&node) < start_times.get(adj) {
                                    edge_types
                                        .insert((node.clone(), adj.clone()), EdgeType::ForwardEdge);
                                } else {
                                    edge_types
                                        .insert((node.clone(), adj.clone()), EdgeType::CrossEdge);
                                }
                            }
                        }
                    }
                    Continuation::End { node } => {
                        time += 1;
                        finished_nodes.insert(node.clone());
                    }
                }
            }
        }

        progress_bar.finish();
        return edge_types;
    }

    // Constructs a Directed Acyclic Graph from the current graph by stripping out the back edges
    pub fn dag(&self) -> DirectedAcyclicGraph<V> {
        let edge_types = self.compute_edge_types();

        let mut graph = AdjacencyGraph::new();

        for ((from, to), edge_type) in edge_types.iter() {
            match edge_type {
                EdgeType::BackEdge => {}
                _ => {
                    graph.add_edge(from.clone(), to.clone());
                }
            }
        }

        return DirectedAcyclicGraph(graph);
    }
}
