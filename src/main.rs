use argh::FromArgs;

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

fn main() {
    let opts = argh::from_env::<CliTool>();

    match opts.nested {
        MySubCommandEnum::Show(show) => {
            let file = std::fs::read_to_string(show.input).expect("cannot read file");
            let entries = parser::parse_source(file.as_str());

            for entry in entries {
                println!("{:?}", entry);
            }
        }
    }
}
