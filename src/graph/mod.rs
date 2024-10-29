use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

pub trait Graph<V>
where
    V: Clone,
{
    fn nodes(&self) -> BTreeSet<V>;
    fn adjacencies(&self) -> BTreeMap<V, BTreeSet<V>>;
    fn edges(&self) -> BTreeSet<(V, V)>;
}

#[derive(Debug)]
pub struct AdjacencyGraph<V>
where
    V: Clone,
{
    nodes: BTreeSet<V>,
    adjacencies: BTreeMap<V, BTreeSet<V>>,
}

#[derive(Debug)]
pub struct UndirectedGraph<V>
where
    V: Clone,
{
    pub directed: AdjacencyGraph<V>,
}

pub mod algorithms;
pub mod edge_types;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    fn print_edge_types<T>(edge_types: &BTreeMap<(T, T), edge_types::EdgeType>)
    where
        T: Debug,
    {
        println!("");
        println!("Edge types:");

        for (edge, edge_type) in edge_types {
            println!("{:?} -> {:?}: {:?}", edge.0, edge.1, edge_type);
        }

        // for (edge_type, edges) in edge_types
        //     .iter()
        //     .fold(BTreeMap::new(), |mut acc, (edge, edge_type)| {
        //         acc.entry(edge_type).or_insert_with(Vec::new).push(edge);
        //         acc
        //     })
        //     .iter()
        // {
        //     println!("- {:?}", edge_type);
        //     for edge in edges {
        //         println!("{:?}", edge);
        //     }
        // }
    }

    #[test]
    fn test_compute_edge_types_cycle() {
        let g = AdjacencyGraph::from_edges(&[(0, 1), (1, 2), (2, 3), (3, 0)]);

        let edge_types = g.compute_edge_types();
        print_edge_types(&edge_types);

        assert_eq!(edge_types.len(), 4);
        assert_eq!(edge_types[&(0, 1)], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&(1, 2)], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&(2, 3)], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&(3, 0)], edge_types::EdgeType::BackEdge);
    }

    #[test]
    fn test_compute_edge_types_forward() {
        let g = AdjacencyGraph::from_edges(&[(0, 1), (1, 2), (0, 2)]);

        let edge_types = g.compute_edge_types();
        print_edge_types(&edge_types);

        assert_eq!(edge_types.len(), 3);
        assert_eq!(edge_types[&(0, 1)], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&(1, 2)], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&(0, 2)], edge_types::EdgeType::ForwardEdge);
    }

    #[test]
    fn test_compute_edge_types_cross() {
        let g = AdjacencyGraph::from_edges(&[(0, 1), (1, 2), (0, 3), (3, 4), (2, 4)]);

        let edge_types = g.compute_edge_types();
        print_edge_types(&edge_types);

        assert_eq!(edge_types.len(), 5);
        assert_eq!(edge_types[&(0, 1)], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&(1, 2)], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&(0, 3)], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&(2, 4)], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&(3, 4)], edge_types::EdgeType::CrossEdge);
    }

    #[test]
    fn test_compute_edge_types_all() {
        let g = AdjacencyGraph::from_edges(&[
            //
            ("u", "v"),
            ("u", "x"),
            ("v", "y"),
            ("y", "x"),
            ("x", "v"),
            ("w", "y"),
            ("w", "z"),
        ]);

        let edge_types = g.compute_edge_types();
        print_edge_types(&edge_types);

        assert_eq!(edge_types.len(), 7);
        assert_eq!(edge_types[&("u", "v")], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&("u", "x")], edge_types::EdgeType::ForwardEdge);
        assert_eq!(edge_types[&("v", "y")], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&("y", "x")], edge_types::EdgeType::TreeEdge);
        assert_eq!(edge_types[&("x", "v")], edge_types::EdgeType::BackEdge);
        assert_eq!(edge_types[&("w", "y")], edge_types::EdgeType::CrossEdge);
        assert_eq!(edge_types[&("w", "z")], edge_types::EdgeType::TreeEdge);
    }

    #[test]
    fn test_compact_chains() {
        let mut g = AdjacencyGraph::from_edges(&[(0, 1), (1, 2), (2, 3), (3, 4)]).undirected();

        println!("Compacting chains...");
        println!("{:?}", g);
        g.compact_chains();
        println!("{:?}", g);
    }
}
