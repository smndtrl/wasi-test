REV="5fc1252"

cd ../wasmtime
git checkout $REV
cargo build -p wasi-preview1-component-adapter --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/wasi_snapshot_preview1.wasm ../wasi-test/
cp -Rf crates/wasi/wit/deps ../wasi-test/wit/
cd ../wasi-test

