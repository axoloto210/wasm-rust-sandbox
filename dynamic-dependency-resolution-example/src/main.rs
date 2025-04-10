use clap::Parser;
use wasmtime::{Engine, Store};
use wasmtime::component::{bindgen, Component, Linker};


use axoloto210::greet::greetable::Host;

bindgen!({
    world:"hello-world",
    path: "../greet/wit"
});

#[derive(Parser, Debug)]
struct Cli {
    wasm_file: String,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = start(cli) {
        println!("{}", e);
    }
}

fn start(cli: Cli) -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker: Linker<Greet> = Linker::new(&engine);

    let mut store:Store<Greet> = Store::new(&engine, Greet::new("Native code".to_string()));

    let component = Component::from_file(&engine, &cli.wasm_file)?;


    HelloWorld::add_to_linker(&mut linker, |greet: &mut Greet| greet)?;

    let hello_world = HelloWorld::instantiate(&mut store, &component, &linker)?;

    let message = hello_world.axoloto210_greet_sayable().call_say(&mut store)?;
    println!("{message}");

    Ok({})
}


struct Greet {
    name: String,
}

impl Greet {
    fn new(name: String) -> Greet{
        Greet {name}
    }
}

// Greetに自動生成されるHostトレイトを実装する。
impl Host for Greet {
    fn name(&mut self)->String{
        self.name.clone()
    }

    fn greet(&mut self, name: String)->String{
        format!("Hello from {name}")
    }
}