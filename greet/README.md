## ライブラリクレート

### WIT WebAssembly Interface Type
インターフェース定義言語。
異なるプログラミング言語間でのインターフェースを定義するための仕様。

#### world 
WITではコンポーネント定義のことをworldと呼ぶ。
`export`で実装するインターフェースを指定。

WITの定義ファイルから生成されたクレートをRustで実装する。
クレートはsrc/bindings.rsに出力される。

### --
`--`は`cargo run`に渡す引数とプログラムに渡す引数の境界を表す。
```
cargo run -- --help
   Compiling greet-user v0.1.0 (/Users/axoloto210/develop/wasm-rust-sandbox/greet-user)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/greet-user --help`
Usage: greet-user <WASM_FILE>

Arguments:
  <WASM_FILE>  

Options:
  -h, --help  Print help
```