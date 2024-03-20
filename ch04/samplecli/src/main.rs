use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "My RPN program",
    version = "1.0.0",
    author = "Hajime Nakamura",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[arg(short, long)]
    verbose: bool,

    #[arg(name="FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified.")
    }
    println!("Is verbosity specified?: {}", opts.verbose)
}
