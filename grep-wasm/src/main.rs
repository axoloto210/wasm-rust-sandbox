use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    file_name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("{cli:?}");
}
