FROM scratch

COPY ./target/wasm32-wasip1/release/hello-wasi-cli.wasm /hello-wasi-cli.wasm

ENTRYPOINT ["/hello-wasi-cli.wasm"]