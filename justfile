wasm:
    cargo build --target wasm32-unknown-unknown --manifest-path packet-analyzer/Cargo.toml
    wasm-bindgen --target bundler --out-dir ./src/wasm target/wasm32-unknown-unknown/debug/packet_analyzer.wasm

wasm-release:
    cargo build --release --target wasm32-unknown-unknown --manifest-path packet-analyzer/Cargo.toml
    wasm-bindgen --target bundler --out-dir ./src/wasm target/wasm32-unknown-unknown/release/packet_analyzer.wasm