use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug)]
pub struct AdjacencyGraph<V>
where
    V: Hash + Eq + Clone,
{
    nodes: HashSet<V>,
    adjacencies: HashMap<V, HashSet<V>>,
}

pub struct UndirectedGraph<V>
where
    V: Hash + Eq + Clone,
{
    graph: AdjacencyGraph<V>,
}

pub mod algorithms;
pub mod edge_types;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_compute_edge_types() {
        let mut g = AdjacencyGraph::new();

        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 4);
        g.add_edge(4, 1);

        let edge_types = g.compute_edge_types();
        let edge_type_dict =
            edge_types
                .iter()
                .fold(BTreeMap::new(), |mut acc, (edge, edge_type)| {
                    acc.entry(edge_type).or_insert_with(Vec::new).push(edge);
                    acc
                });

        for (edge_type, edges) in edge_type_dict.iter() {
            println!("- {:?}", edge_type);
            for edge in edges {
                println!("Edge: {:?}", edge);
            }
        }
    }
}
