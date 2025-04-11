use clap::Parser;
use wasmtime::component::{Component, Linker, ResourceTable, bindgen};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi::IoView;

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

    let mut store: Store<Greet> = Store::new(&engine, Greet::new("Native code".to_string()));

    let component = Component::from_file(&engine, &cli.wasm_file)?;

    HelloWorld::add_to_linker(&mut linker, |greet: &mut Greet| greet)?;

    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let hello_world = HelloWorld::instantiate(&mut store, &component, &linker)?;

    

    let message = hello_world
        .axoloto210_greet_sayable()
        .call_say(&mut store)?;
    println!("{message}");

    Ok({})
}

struct Greet {
    name: String,
    wasi_ctx: WasiCtx, // 環境変数のリストや標準入出力のストリーム、Wasmコンポーネントからのアクセスが許されているフォルダーのリストなどを保持。
    resource_table: ResourceTable,
}

impl Greet {
    fn new(name: String) -> Greet {

        let wasi_ctx = WasiCtxBuilder::new().build();

        let resource_table = ResourceTable::new();

        Greet {
            name,
            wasi_ctx,
            resource_table,
        }
    }
}


impl IoView for Greet {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
    

}

impl WasiView for Greet {

    fn ctx (&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

// Greetに自動生成されるHostトレイトを実装する。
impl Host for Greet {
    fn name(&mut self) -> String {
        self.name.clone()
    }

    fn greet(&mut self, name: String) -> String {
        format!("Hello from {name}")
    }
}
