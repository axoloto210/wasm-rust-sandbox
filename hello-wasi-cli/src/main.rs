use ferris_says::say;

fn main() {
    if let Err(e) = say("Hello, Wasm!", 80, &mut std::io::stdout()) {
        println!("{e}");
    }
}
