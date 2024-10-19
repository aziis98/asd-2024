use std::{collections::HashMap, hash::Hash};

struct GraphEdge {
    from: u32,
    to: u32,
}

pub struct Graph<V, E>
where
    V: Hash + Eq + Clone,
{
    nodes: HashMap<V, u32>,
    edges: HashMap<(u32, u32), E>,

    adjacency_list: HashMap<u32, Vec<(u32, u32)>>,
}
