## wasmtime とサンドボックス

Wasmtime が Wasm コンポーネントをシェルが実行される環境から隔離された環境（サンドボックス）で実行させるため、そのままファイル名を引数に渡してもファイルにアクセスできない。
`wasmtime target/wasm32-wasip1/debug/grep-wasm.wasm foo src/main.rs`

```
Error: failed to find a pre-opened file descriptor through which "src/main.rs" could be opened
```

`wasmtime`ではアクセスできるディレクトリを`--dir .`のように指定する必要がある。
`wasmtime --dir .  target/wasm32-wasip1/debug/grep-wasm.wasm foo src/main.rs`

### Rust での Wasm 向け grep コマンド

`wasmtime --dir .  target/wasm32-wasip1/debug/grep-wasm.wasm print src/main.r
s`

```
println!("{line}")
println!("Error: {e}")
```
