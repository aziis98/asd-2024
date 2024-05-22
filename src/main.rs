use argh::FromArgs;
use gfa::{Entry, Orientation};
use graph::AdjacencyGraph;

mod gfa;
mod graph;
mod parser;

#[derive(FromArgs, PartialEq, Debug)]
/// Strumento CLI per il progetto di Algoritmi e Strutture Dati 2024
struct CliTool {
    #[argh(subcommand)]
    nested: MySubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum MySubCommandEnum {
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
        MySubCommandEnum::Show(show) => {
            let file = std::fs::File::open(show.input)?;
            let entries = parser::parse_source(file)?;

            let mut graph = AdjacencyGraph::new();

            for entry in entries {
                println!("{:?}", entry);

                match entry {
                    Entry::Segment { id, sequence } => {
                        graph.add_node(id, sequence);
                    }
                    Entry::Link {
                        from,
                        from_orient,
                        to,
                        to_orient,
                    } => match (from_orient, to_orient) {
                        (Orientation::Forward, Orientation::Forward)
                        | (Orientation::Reverse, Orientation::Reverse) => {
                            graph.add_edge(from, to);
                        }
                        (Orientation::Forward, Orientation::Reverse)
                        | (Orientation::Reverse, Orientation::Forward) => {
                            graph.add_edge(to, from);
                        }
                    },
                    _ => {}
                }
            }

            for (from, adjacencies) in graph.adjacencies().iter() {
                println!(
                    "{} -> {}",
                    from,
                    adjacencies
                        .iter()
                        .map(|to| to.to_owned())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }

            let cc = graph.compute_ccs_2();
            println!("CCs: {:?}", cc);
            println!("Number of connected components: {}", cc.len());
        }
    }

    Ok(())
}
