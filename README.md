# Awqward Bomberman

Bomberman made in a day using Rust with the Bevy game engine.

## Building to WASM

Start by making sure you have `wasm` as a compilation target.

```shell
rustup target install wasm32-unknown-unknown
```

### Development

For some reason gamepad support only works in the browser.
See [https://bevy-cheatbook.github.io/platforms/wasm.html](cheatbook) for how this works.

To run this locally you need to install `wasm-server-runner`.

```shell
cargo install wasm-server-runner
```

Now to run it locally:

```shell
cargo run --target wasm32-unknown-unknown
```

### Release

To create a release build you can do the following:

```shell
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/
```
