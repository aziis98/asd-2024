use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
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

impl<V> AdjacencyGraph<V>
where
    V: Hash + Eq + Clone + Debug,
{
    pub fn compute_edge_types(&self) -> HashMap<(&V, &V), EdgeType> {
        /// To correctly compute the start and end times of the nodes in the
        /// graph, we need to keep do work before and after the recursion call
        enum RecurseState<'a, V> {
            Before(&'a V),
            BeforeNeighbor(&'a V, &'a V),
            AfterNeighbor(&'a V),
        }

        let mut edge_types = HashMap::new();

        let mut visited = HashSet::new();
        let mut start_times = HashMap::new();
        let mut finished_nodes = HashSet::new();

        let mut time = 0;

        let progress_bar = ProgressBar::new(self.nodes().len() as u64);

        for node in self.nodes().iter() {
            if visited.contains(node) {
                continue;
            }

            let mut stack = Vec::new();

            stack.push(RecurseState::Before(node));

            while let Some(state) = stack.pop() {
                match state {
                    RecurseState::Before(node) => {
                        progress_bar.inc(1);
                        visited.insert(node.clone());
                        start_times.insert(node, time);
                        time += 1;

                        // this is extremely important that is before the adjacencies to correctly
                        // iterate over the graph

                        if let Some(adjacencies) = self.get_adjacencies(node) {
                            for adj in adjacencies {
                                println!("Node: {:?} Adj: {:?}", node, adj,);

                                stack.push(RecurseState::AfterNeighbor(node));

                                if !visited.contains(adj) {
                                    edge_types.insert((node, adj), EdgeType::TreeEdge);
                                    stack.push(RecurseState::Before(adj));
                                } else {
                                    stack.push(RecurseState::BeforeNeighbor(node, adj));
                                }
                            }
                        }
                    }
                    RecurseState::AfterNeighbor(node) => {
                        finished_nodes.insert(node, time);
                        time += 1;
                    }
                    RecurseState::BeforeNeighbor(node, adj) => {
                        let start_time_node = start_times.get(node).unwrap();
                        let start_time_adj = start_times.get(adj).unwrap();
                        let end_time_node = finished_nodes.get(node).unwrap_or(&0);
                        let end_time_adj = finished_nodes.get(adj).unwrap_or(&0);

                        println!(
                            "Times: ({:?}, {:?}) ({:?}, {:?})",
                            start_time_node, end_time_node, start_time_adj, end_time_adj
                        );

                        match (
                            start_time_node.cmp(start_time_adj),
                            end_time_node.cmp(end_time_adj),
                        ) {
                            (Ordering::Less, Ordering::Greater) => {
                                edge_types.insert((node, adj), EdgeType::ForwardEdge);
                            }
                            (Ordering::Greater, Ordering::Less) => {
                                edge_types.insert((node, adj), EdgeType::BackEdge);
                            }
                            _ => {
                                edge_types.insert((node, adj), EdgeType::CrossEdge);
                            }
                        }
                    }
                }
            }
        }

        edge_types
    }
}
