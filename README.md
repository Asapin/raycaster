Simple Wolfenstein-style raycaster in Rust + WASM + Wasm-4

## How to run
* Install WASM32 toolchain: `rustup target add wasm32-unknown-unknown`
* Download Wasm-4 fantasy game console: <https://wasm4.org/docs/>
* Build the project: `cargo build --release`
* Run the game: `.\w4.exe run-native .\target\wasm32-unknown-unknown\release\raycaster.wasm`

## Controls

You can move with arrow keys.
