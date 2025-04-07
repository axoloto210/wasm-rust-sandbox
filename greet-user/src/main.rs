use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    wasm_file: String
}



fn main() {
    let args = Args::parse();
    println!("{:?}",args);
}
