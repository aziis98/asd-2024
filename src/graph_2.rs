// use std::{collections::HashMap, io::Read};

// use crate::{gfa::Entry, parser};

// pub struct Graph {
//     nodes: HashMap<String, usize>,

//     edges_from: Vec<usize>,
//     edges_to: Vec<usize>,
// }

// #[derive(Debug)]
// pub enum GraphError {
//     NodeNotFound(String),
// }

// impl Graph {
//     pub fn new() -> Self {
//         Self {
//             nodes: HashMap::new(),
//             edges_from: Vec::new(),
//             edges_to: Vec::new(),
//         }
//     }

//     pub fn add_node(&mut self, id: String) {
//         if self.nodes.contains_key(&id) {
//             return;
//         }

//         self.nodes.insert(id, self.nodes.len());
//     }

//     pub fn add_edge(&mut self, from_id: &String, to_id: &String) -> Result<(), GraphError> {
//         let from = self
//             .nodes
//             .get(from_id)
//             .ok_or(GraphError::NodeNotFound(from_id.clone()))?;

//         let to = self
//             .nodes
//             .get(to_id)
//             .ok_or(GraphError::NodeNotFound(to_id.clone()))?;

//         self.edges_from.push(*from);
//         self.edges_to.push(*to);

//         Ok(())
//     }
// }

// #[derive(Debug)]
// pub enum LoadGraphError {
//     IoError(std::io::Error),
//     GraphError(GraphError),
// }

// pub fn load_graph<R: Read>(reader: R, len: u64) -> Result<Graph, LoadGraphError> {
//     println!("Loading graph");

//     let mut graph = Graph::new();

//     let entries = parser::parse_source(reader, len).map_err(|e| LoadGraphError::IoError(e))?;

//     let node_count = entries
//         .iter()
//         .filter_map(|entry| match entry {
//             Entry::Segment { id, .. } => Some(id),
//             _ => None,
//         })
//         .count();

//     println!("Node count: {}", node_count);

//     for entry in entries
//         .iter()
//         .filter(|entry| matches!(entry, Entry::Link { .. }))
//     {
//         if let Entry::Link {
//             from,
//             from_orient,
//             to,
//             to_orient,
//         } = entry
//         {
//             let node_from = format!("{}{}", from, from_orient);
//             let node_to = format!("{}{}", to, to_orient);

//             graph.add_node(node_from.clone());
//             graph.add_node(node_to.clone());

//             graph
//                 .add_edge(&node_from, &node_to)
//                 .expect("Error adding edge");
//             graph
//                 .add_edge(&node_to, &node_from)
//                 .expect("Error adding edge");
//         }
//     }

//     println!("Loading completed");

//     Ok(graph)
// }
