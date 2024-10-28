use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use indicatif::ProgressBar;

use super::AdjacencyGraph;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

        if let Some(adjacencies) = graph.get_adjacencies(node) {
            for adj in adjacencies.iter() {
                if !self.visited.contains(adj) {
                    self.dfs(graph, adj, Some(node));
                } else {
                    if !self.finished_nodes.contains(adj) {
                        self.edge_types
                            .insert((node.clone(), adj.clone()), EdgeType::BackEdge);
                    } else if self.start_times.get(node) < self.start_times.get(adj) {
                        self.edge_types
                            .insert((node.clone(), adj.clone()), EdgeType::ForwardEdge);
                    } else {
                        self.edge_types
                            .insert((node.clone(), adj.clone()), EdgeType::CrossEdge);
                    }
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

    // pub fn compute_edge_types(&self) -> BTreeMap<(V, V), EdgeType> {
    //     println!("{:?}", self);

    //     let mut edge_types: BTreeMap<(V, V), EdgeType> = BTreeMap::new();
    //     let mut visited: BTreeSet<V> = BTreeSet::new();

    //     let mut start_times: BTreeMap<V, i32> = BTreeMap::new();
    //     let mut finished_nodes: BTreeSet<V> = BTreeSet::new();

    //     #[derive(Debug)]
    //     enum RecurseState<V> {
    //         Visit { node: V, parent: Option<V> },
    //         End { node: V },
    //     }

    //     let mut time = 0;

    //     // let progress_bar = ProgressBar::new(self.nodes().len() as u64);

    //     for start in self.nodes().iter() {
    //         if visited.contains(start) {
    //             continue;
    //         }

    //         let mut stack: Vec<RecurseState<V>> = Vec::new();

    //         // The first node does not have a parent
    //         stack.push(RecurseState::End {
    //             node: start.clone(),
    //         });
    //         stack.push(RecurseState::Visit {
    //             node: start.clone(),
    //             parent: None,
    //         });

    //         println!("Starting DFS from {:?}", start);

    //         while let Some(state) = stack.pop() {
    //             println!("Current: {:?}", state);
    //             println!("Finished Nodes: {:?}", finished_nodes);

    //             match state {
    //                 RecurseState::Visit { node, parent } => {
    //                     if visited.contains(&node) {
    //                         // progress_bar.inc(1);
    //                     }

    //                     if let Some(parent) = parent.clone() {
    //                         if !visited.contains(&node) {
    //                             println!("{:?} => TreeEdge", (parent.clone(), node.clone()));
    //                             edge_types
    //                                 .insert((parent.clone(), node.clone()), EdgeType::TreeEdge);
    //                         } else {
    //                             if !finished_nodes.contains(&parent) {
    //                                 println!("{:?} => BackEdge", (parent.clone(), node.clone()));
    //                                 edge_types
    //                                     .insert((node.clone(), parent.clone()), EdgeType::BackEdge);
    //                             } else if start_times.get(&node) < start_times.get(&parent) {
    //                                 println!("{:?} => ForwardEdge", (parent.clone(), node.clone()));
    //                                 edge_types.insert(
    //                                     (node.clone(), parent.clone()),
    //                                     EdgeType::ForwardEdge,
    //                                 );
    //                             } else {
    //                                 println!("{:?} => CrossEdge", (parent.clone(), node.clone()));
    //                                 edge_types.insert(
    //                                     (node.clone(), parent.clone()),
    //                                     EdgeType::CrossEdge,
    //                                 );
    //                             }
    //                         }
    //                     }

    //                     time += 1;
    //                     start_times.insert(node.clone(), time);

    //                     visited.insert(node.clone());

    //                     // it is extremely important that this before the adjacencies to correctly
    //                     // iterate over the graph
    //                     // stack.push(RecurseState::AfterNeighbors { node });

    //                     if let Some(adjacencies) = self.get_adjacencies(&node) {
    //                         println!("adjacencies: {:?}", adjacencies);
    //                         for adj in adjacencies.iter().rev() {
    //                             if !visited.contains(&adj) {
    //                                 stack.push(RecurseState::End { node: adj.clone() });
    //                                 stack.push(RecurseState::Visit {
    //                                     node: adj.clone(),
    //                                     parent: Some(node.clone()),
    //                                 });
    //                             }
    //                         }
    //                     }
    //                 }
    //                 RecurseState::End { node } => {
    //                     time += 1;
    //                     finished_nodes.insert(node.clone());
    //                 }
    //             }

    //             println!();

    //             // println!("after:");
    //             // println!("~> {:?}", stack);
    //         }
    //     }

    //     // progress_bar.finish();

    //     return edge_types;
    // }
}
