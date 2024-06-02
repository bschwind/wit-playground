cargo build -p model-code --release --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/release/model_code.wasm -o my-component.wasm
