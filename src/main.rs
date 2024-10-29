#![allow(dead_code)]

mod gfa;
mod graph;

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    io::{BufRead, BufReader},
    process,
};

use argh::FromArgs;
use gfa::{Entry, Orientation};
use graph::{AdjacencyGraph, Graph};
use indicatif::ProgressIterator;
use rand::seq::SliceRandom;

#[derive(FromArgs, PartialEq, Debug)]
/// Strumento CLI per il progetto di Algoritmi e Strutture Dati 2024
struct CliTool {
    #[argh(subcommand)]
    nested: CliSubcommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum CliSubcommands {
    Show(CommandShow),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Parse and show the content of a file
#[argh(subcommand, name = "show")]
struct CommandShow {
    #[argh(option, short = 'i')]
    /// file to read
    input: String,
}

fn main() -> std::io::Result<()> {
    let opts = argh::from_env::<CliTool>();

    match opts.nested {
        CliSubcommands::Show(show) => {
            let file_lines_count = BufReader::new(std::fs::File::open(&show.input)?)
                .lines()
                .progress_with(
                    indicatif::ProgressBar::new_spinner().with_message("estimating line count"),
                )
                .count() as u64;

            let file = std::fs::File::open(show.input)?;

            let entries = gfa::parser::parse_source(file, file_lines_count)?;

            println!("Number of entries: {}", entries.len());

            let mut sequence_map = HashMap::new();
            let mut graph: AdjacencyGraph<(String, Orientation)> = AdjacencyGraph::new();

            for entry in entries {
                match entry {
                    Entry::Segment { id, sequence } => {
                        sequence_map.insert(id.clone(), sequence);
                    }
                    Entry::Link {
                        from,
                        from_orient,
                        to,
                        to_orient,
                    } => {
                        graph.add_edge((from.clone(), from_orient), (to.clone(), to_orient));
                    }
                    _ => {}
                }
            }

            compute_graph_stats(&graph);

            let edge_types = compute_edge_types(&graph);

            println!("Removing back edges...");

            for ((from, to), edge_type) in edge_types.iter() {
                match edge_type {
                    graph::edge_types::EdgeType::BackEdge => {
                        graph.remove_edge(from, to);
                    }
                    _ => {}
                }
            }

            compute_edge_types(&graph);

            let ccs = compute_ccs(&graph);

            println!("Picking largest connected component...");
            // pick the largest connected component
            let largest_cc = ccs
                .iter()
                .max_by_key(|cc| cc.len())
                .expect("at least one connected components");

            let largest_cc_graph = graph.restricted(largest_cc);

            compute_graph_stats(&largest_cc_graph);
            compute_edge_types(&largest_cc_graph);

            // let mut largest_cc_graph = graph.restricted(largest_cc).undirected();

            // compute_graph_stats(&largest_cc_graph);
            // // compute_edge_types(&largest_cc_graph);

            // println!("Compacting chains...");
            // largest_cc_graph.compact_chains();

            // compute_graph_stats(&largest_cc_graph);
            // compute_edge_types(&largest_cc_graph.directed);

            println!("Cleaning up...");
            process::exit(0);
        }
    }
}

fn compute_ccs<V>(graph: &AdjacencyGraph<V>) -> Vec<Vec<V>>
where
    V: Ord + Eq + Clone + Debug,
{
    println!("Computing connected components...");
    let ccs = graph.undirected().connected_components();

    println!("Computing sizes histogram...");
    let hist: BTreeMap<_, _> = ccs
        .iter()
        .map(|cc| cc.len()) // map to size of each cc
        .fold(BTreeMap::new(), |mut acc, len| {
            *acc.entry(len).or_insert(0) += 1;
            acc
        });

    println!("Connected components histogram (size/count):");
    for (size, count) in hist.iter() {
        println!("- {}: {}", size, count);
    }

    ccs
}

fn compute_edge_types<V>(graph: &AdjacencyGraph<V>) -> BTreeMap<(V, V), graph::edge_types::EdgeType>
where
    V: Ord + Eq + Clone + Debug,
{
    println!("Computing edge types...");
    let edge_types = graph.compute_edge_types();

    println!("Computing edge types histogram...");
    let histogram = edge_types.iter().map(|(_, edge_type)| edge_type).fold(
        BTreeMap::new(),
        |mut acc, edge_type| {
            *acc.entry(edge_type.clone()).or_insert(0) += 1;
            acc
        },
    );

    println!(
        "Edge count: {}, Total edge count: {}",
        graph.edges().count(),
        edge_types.len()
    );
    println!("Edge types histogram (type/count):");
    for (edge_type, count) in histogram.iter() {
        println!("- {:?}: {}", edge_type, count);
    }

    edge_types
}

fn compute_shuffled_graph<V>(graph: &AdjacencyGraph<V>) -> AdjacencyGraph<V>
where
    V: Ord + Eq + Clone + Debug,
{
    println!("Shuffling graph...");

    let mut g2 = AdjacencyGraph::new();

    let mut shuffled_nodes: Vec<_> = graph.nodes().iter().collect::<Vec<_>>();
    shuffled_nodes.shuffle(&mut rand::thread_rng());

    for node in shuffled_nodes.iter() {
        g2.add_node((*node).clone());
    }

    let mut shuffled_map = BTreeMap::new();
    for (i, node) in graph.nodes().iter().enumerate() {
        shuffled_map.insert(node.clone(), shuffled_nodes[i].clone());
    }

    for edge in graph.edges() {
        g2.add_edge(
            shuffled_map.get(&edge.0).unwrap().clone(),
            shuffled_map.get(&edge.1).unwrap().clone(),
        );
    }

    compute_edge_types(&g2);

    g2
}

/// This function prints the number of nodes, edges and a histogram of the degrees of the nodes
/// in the graph (computing the degrees might take a long time)
fn compute_graph_stats<V>(graph: &impl Graph<V>)
where
    V: Ord + Eq + Clone + Debug,
{
    println!("Computing graph stats...");

    let mut vertices_degrees = BTreeMap::new();
    let mut vertices_in_degrees = BTreeMap::new();
    let mut vertices_out_degrees = BTreeMap::new();

    println!("Computing nodes degrees...");

    let progress_bar = indicatif::ProgressBar::new(graph.edges().len() as u64);

    for (from, tos) in graph.adjacencies() {
        *vertices_degrees.entry(from).or_insert(0) += tos.len();
        *vertices_out_degrees.entry(from).or_insert(0) += tos.len();

        for to in tos {
            progress_bar.inc(1);

            *vertices_degrees.entry(to).or_insert(0) += 1;
            *vertices_in_degrees.entry(to).or_insert(0) += 1;
        }
    }

    progress_bar.finish();

    println!("Computing histogram...");
    let histogram: BTreeMap<usize, usize> = vertices_degrees
        .iter()
        .map(|(_, degree)| *degree)
        .fold(BTreeMap::new(), |mut acc, degree| {
            *acc.entry(degree).or_insert(0) += 1;
            acc
        });

    println!("Stats:");
    println!("- Nodes: {}", graph.nodes().len());
    println!("- Edges: {}", graph.edges().len());

    println!("Graph degrees histogram (degree/count):");
    for (degree, count) in histogram.iter() {
        println!("- {}: {}", degree, count);
    }
}
