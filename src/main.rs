#![allow(dead_code)]

mod gfa;
mod graph;
mod rolling_hash;

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    io::{BufRead, BufReader},
    process,
};

use argh::FromArgs;
use gfa::{Entry, Orientation};
use graph::{AdjacencyGraph, DirectedAcyclicGraph, Graph};
use indicatif::ProgressIterator;
use rand::seq::SliceRandom;
use rolling_hash::RollingHasher;

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

    #[argh(option, short = 'c', default = "1")]
    /// number of paths to visit
    path_count: usize,

    #[argh(option, short = 'p', default = "\"ACGT\".to_string()")]
    /// k-mer pattern to search
    pattern: String,

    #[argh(option, short = 'k', default = "4")]
    /// k-mer length
    kmer_size: usize,
}

fn main() -> std::io::Result<()> {
    let opts = argh::from_env::<CliTool>();

    match opts.nested {
        CliSubcommands::Show(opts) => {
            // validate opts.pattern is a valid DNA sequence
            if opts.pattern.chars().any(|c| !"ACGT".contains(c)) {
                eprintln!("Invalid pattern: {:?}", opts.pattern);
                process::exit(1);
            }

            println!("Estimating line count...");

            let file_lines_count = BufReader::new(std::fs::File::open(&opts.input)?)
                .lines()
                .progress_with(indicatif::ProgressBar::new_spinner())
                .count() as u64;

            let entries =
                gfa::parser::parse_source(std::fs::File::open(opts.input)?, file_lines_count)?;

            println!("Number of entries: {}", entries.len());

            let mut sequence_map = HashMap::new();
            let mut graph: AdjacencyGraph<(String, Orientation)> = AdjacencyGraph::new();

            let mut invalid_nodes = vec![];

            for entry in entries {
                match entry {
                    Entry::Segment { id, sequence } => {
                        // validate sequence is a valid DNA sequence
                        if sequence.chars().any(|c| !"ACGT".contains(c)) {
                            invalid_nodes.push(id.clone());
                            continue;
                        }

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

            println!("Removing {} invalid nodes...", invalid_nodes.len());
            // remove invalid nodes
            for id in invalid_nodes.iter().progress() {
                graph.remove_node(&(id.clone(), Orientation::Forward));
                graph.remove_node(&(id.clone(), Orientation::Reverse));
            }
            println!();

            compute_graph_degrees(&graph);

            let dag = graph.dag();

            compute_edge_types(&dag.0);

            let ccs = compute_ccs(&dag.0);

            println!("Picking largest connected component...");
            // pick the largest connected component
            let largest_cc = ccs
                .iter()
                .max_by_key(|cc| cc.len())
                .expect("at least one connected components");

            let largest_cc_graph = dag.restricted(largest_cc);

            let degrees = compute_graph_degrees(&largest_cc_graph);
            compute_edge_types(&largest_cc_graph); // to double check this is a DAG

            println!("Searching for a start node...");
            let start_node = degrees
                .iter()
                .find(|(_, degree)| degree.in_degree == 0)
                .expect("no start node found")
                .0;

            println!("Start node: {:?}", start_node);
            println!("{:?}", degrees.get(start_node).unwrap());

            compute_orientation_histogram(&largest_cc_graph);

            println!("Visiting the graph, searching {} paths...", opts.path_count);

            let sequences = compute_sequences(
                &sequence_map,
                &largest_cc_graph,
                start_node,
                opts.path_count,
            );

            for (i, sequence) in sequences.iter().enumerate() {
                println!("Sequence #{} of length {}", i + 1, sequence.len());

                println!("Searching {} (naive)...", opts.pattern);
                println!(
                    "Occurrences: {:?}\n",
                    compute_sequence_occurrences_naive(sequence, &opts.pattern)
                );

                println!("Searching {} (rolling hash)...", opts.pattern);
                println!(
                    "Occurrences: {:?}\n",
                    compute_sequence_occurrences_rolling_hash(sequence, &opts.pattern)
                );
            }

            compute_kmer_histogram_lb(&sequence_map, &largest_cc_graph, opts.kmer_size);

            println!("Cleaning up...");
            process::exit(0);
        }
    }
}

fn compute_kmer_histogram_lb(
    sequence_map: &HashMap<String, String>,
    graph: &DirectedAcyclicGraph<(String, Orientation)>,
    k: usize,
) {
    println!("Computing k-mer histogram...");

    let mut kmer_counts = HashMap::new();

    for node in graph.nodes().iter().progress() {
        let sequence = get_node_sequence(sequence_map, &node);
        let kmer_counts_node = sequence_kmer_histogram(&sequence, k);

        for (kmer, count) in kmer_counts_node {
            *kmer_counts.entry(kmer).or_insert(0) += count;
        }
    }

    let mut kmer_counts = kmer_counts.into_iter().collect::<Vec<_>>();

    kmer_counts.sort_by(|a, b| b.1.cmp(&a.1).reverse());

    println!("K-mer histogram (kmers/count):");
    for (kmer, count) in &kmer_counts {
        println!("- {}: {}", kmer, count);
    }

    println!(
        "Found {} of {} possible kmers (about {:.2}% coverage)",
        kmer_counts.len(),
        4usize.pow(k as u32),
        (kmer_counts.len() as f64 / 4usize.pow(k as u32) as f64) * 100.0
    );
}

fn sequence_kmer_histogram(sequence: &str, k: usize) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    if sequence.len() < k {
        return counts;
    }

    for i in 0..sequence.len() - k {
        let kmer = &sequence[i..i + k];
        *counts.entry(kmer.to_string()).or_insert(0) += 1;
    }

    counts
}

fn letter_to_number(letter: char) -> u64 {
    match letter {
        'A' => 1,
        'C' => 2,
        'G' => 3,
        'T' => 4,
        _ => panic!("Invalid letter: {}", letter),
    }
}

fn letter_complement(letter: char) -> char {
    match letter {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => panic!("Invalid letter: {}", letter),
    }
}

fn get_node_sequence<'a>(
    sequence_map: &'a HashMap<String, String>,
    node: &(String, Orientation),
) -> String {
    let (id, orientation) = node;
    let seq = sequence_map.get(id).expect("sequence not found");

    match orientation {
        Orientation::Forward => seq.clone(),
        Orientation::Reverse => seq.chars().map(letter_complement).rev().collect::<String>(),
    }
}

fn compute_sequence_occurrences_rolling_hash(sequence: &str, pattern: &str) -> Vec<usize> {
    let chars = sequence.chars().map(letter_to_number).collect::<Vec<_>>();

    let mut occurrences = vec![];

    let mut rl = RollingHasher::new(3000, 5);
    // let mut rl = RollingHash::new(1_000_000, 5);

    let pattern_hash = rl.hash_pattern(&pattern.chars().map(letter_to_number).collect::<Vec<_>>());

    for i in 0..pattern.len() {
        rl.add_last(chars[i]);
    }

    for i in pattern.len()..sequence.len() {
        let hash = rl.hash();

        if rl.compare(&hash, &pattern_hash) {
            println!("Hash match at position {}", i);

            // check for false positives
            if &sequence[i - pattern.len()..i] != pattern {
                println!("=> False positive");
            } else {
                println!("=> Correct");
                occurrences.push(i - pattern.len());
            }
        }

        rl.advance(chars[i]);
    }

    occurrences
}

fn compute_sequence_occurrences_naive(sequence: &str, pattern: &str) -> Vec<usize> {
    let mut occurrences = vec![];

    for i in 0..sequence.len() - pattern.len() {
        if &sequence[i..i + pattern.len()] == pattern {
            occurrences.push(i);
        }
    }

    occurrences
}

fn compute_sequences(
    sequence_map: &HashMap<String, String>,
    graph: &DirectedAcyclicGraph<(String, Orientation)>,
    start_node: &(String, Orientation),
    count: usize,
) -> Vec<String> {
    let mut sequences = vec![];

    let mut path_counter = 0;
    graph.all_paths(start_node, |path| {
        println!("Path #{} of length {}", path_counter + 1, path.len());

        let mut sequence = String::new();
        for node in path {
            let piece = get_node_sequence(sequence_map, &node);
            sequence.push_str(&piece);
        }

        sequences.push(sequence);

        path_counter += 1;
        path_counter < count
    });

    sequences
}

fn compute_orientation_histogram(graph: &impl Graph<(String, Orientation)>) {
    let orientation_histogram =
        graph
            .nodes()
            .iter()
            .map(|node| node.1)
            .fold(BTreeMap::new(), |mut acc, orientation| {
                *acc.entry(orientation).or_insert(0) += 1;
                acc
            });

    println!("Orientation histogram:");
    for (orientation, count) in orientation_histogram.iter() {
        println!("- {:?}: {}", orientation, count);
    }
    println!();
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
    println!();

    ccs
}

fn compute_edge_types<V>(graph: &impl Graph<V>) -> BTreeMap<(V, V), graph::edge_types::EdgeType>
where
    V: Ord + Eq + Clone + Debug,
{
    println!("Computing edge types...");
    let edge_types = graph.to_adjecency_graph().compute_edge_types();

    println!("Computing edge types histogram...");
    let histogram = edge_types.iter().map(|(_, edge_type)| edge_type).fold(
        BTreeMap::new(),
        |mut acc, edge_type| {
            *acc.entry(edge_type.clone()).or_insert(0) += 1;
            acc
        },
    );

    println!("Node count: {}", graph.nodes().len());
    println!(
        "Edge count: {}, Total edge count: {}",
        graph.edges().len(),
        edge_types.len()
    );

    println!("Edge types histogram (type/count):");
    for (edge_type, count) in histogram.iter() {
        println!("- {:?}: {}", edge_type, count);
    }
    println!();

    edge_types
}

fn compute_shuffled_graph<V>(graph: &AdjacencyGraph<V>) -> AdjacencyGraph<V>
where
    V: Ord + Eq + Clone + Debug,
{
    println!("Shuffling graph...");

    let mut g2 = AdjacencyGraph::new();

    let mut shuffled_nodes: Vec<_> = graph.nodes().into_iter().collect::<Vec<_>>();
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

#[derive(Debug)]
struct NodeDegree {
    degree: usize,
    in_degree: usize,
    out_degree: usize,
}

/// This function prints the number of nodes, edges and a histogram of the degrees of the nodes
/// in the graph (computing the degrees might take a long time)
fn compute_graph_degrees<V>(graph: &impl Graph<V>) -> BTreeMap<V, NodeDegree>
where
    V: Ord + Eq + Clone + Debug,
{
    println!("Computing graph stats...");

    let mut vertices_degrees = BTreeMap::new();
    let mut vertices_in_degrees = BTreeMap::new();
    let mut vertices_out_degrees = BTreeMap::new();

    println!("Computing nodes degrees...");

    let progress_bar = indicatif::ProgressBar::new(graph.edges().len() as u64);

    for node in graph.nodes() {
        vertices_degrees.insert(node.clone(), 0);
        vertices_in_degrees.insert(node.clone(), 0);
        vertices_out_degrees.insert(node.clone(), 0);
    }

    for (from, tos) in graph.adjacencies() {
        *vertices_degrees.entry(from.clone()).or_insert(0) += tos.len();
        *vertices_out_degrees.entry(from.clone()).or_insert(0) += tos.len();

        for to in tos {
            progress_bar.inc(1);

            *vertices_degrees.entry(to.clone()).or_insert(0) += 1;
            *vertices_in_degrees.entry(to.clone()).or_insert(0) += 1;
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
    let histogram_in: BTreeMap<usize, usize> = vertices_in_degrees
        .iter()
        .map(|(_, degree)| *degree)
        .fold(BTreeMap::new(), |mut acc, degree| {
            *acc.entry(degree).or_insert(0) += 1;
            acc
        });
    let histogram_out: BTreeMap<usize, usize> = vertices_out_degrees
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
    println!("In-degrees histogram (degree/count):");
    for (degree, count) in histogram_in.iter() {
        println!("- {}: {}", degree, count);
    }
    println!("Out-degrees histogram (degree/count):");
    for (degree, count) in histogram_out.iter() {
        println!("- {}: {}", degree, count);
    }
    println!();

    graph
        .nodes()
        .iter()
        .map(|node| {
            (
                node.clone(),
                NodeDegree {
                    degree: *vertices_degrees.get(node).expect("node already computed"),
                    in_degree: *vertices_in_degrees
                        .get(node)
                        .expect("node already computed"),
                    out_degree: *vertices_out_degrees
                        .get(node)
                        .expect("node already computed"),
                },
            )
        })
        .collect()
}

// fn compute_compact_graph<V>(graph: &mut UndirectedGraph<V>) -> UndirectedGraph<V>
// where
//     V: Ord + Eq + Clone + Debug,
// {
//     compute_graph_degrees(graph);
//     // compute_edge_types(graph);

//     println!("Compacting chains...");
//     graph.compact_chains();

//     compute_graph_degrees(graph);
//     compute_edge_types(graph.directed);

//     graph.clone()
// }
