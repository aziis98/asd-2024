use std::{
    collections::{BTreeMap, HashMap},
    io::{BufRead, BufReader},
};

use argh::FromArgs;
use gfa::{Entry, Orientation};
use graph::AdjacencyGraph;
use indicatif::ProgressIterator;

mod gfa;
mod graph;
mod parser;

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

            let entries = parser::parse_source(file, file_lines_count)?;

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

            // Print the graph
            // for ((from, orient), adjacencies) in graph.adjacencies().iter() {
            //     println!(
            //         "{}{} -> {}",
            //         from,
            //         orient,
            //         adjacencies
            //             .iter()
            //             .map(|(to, orient)| format!("{}{}", to, orient))
            //             .collect::<Vec<String>>()
            //             .join(", ")
            //     );
            // }

            // let cc = graph.compute_ccs();

            // println!("CCs: {:?}", cc);
            // println!("Number of connected components: {}", cc.len());

            // graph.print_stats();

            println!("Graph has cycles: {}", graph.is_cyclic());

            let edge_types = graph.compute_edge_types();

            let edge_type_histogram: BTreeMap<_, _> = edge_types
                .iter()
                .map(|(_, edge_type)| edge_type)
                .fold(BTreeMap::new(), |mut acc, edge_type| {
                    *acc.entry(edge_type).or_insert(0) += 1;
                    acc
                });

            println!("Edge types histogram: {:?}", edge_type_histogram);

            println!("Cleaning up...");
        }
    }

    Ok(())
}
