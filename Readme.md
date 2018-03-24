# Snake
This is a snake clone written in Rust with WebAssembly using the stdweb crate.

# Usage
Move with the arrow keys.

# Building
## Install Rust
```sh
$ curl https://sh.rustup.rs -sSf | sh
```
## Install the `wasm32-unknown-unknown` target
```sh
$ rustup target install wasm32-unknown-unknown
```
## Install `cargo-web`
```sh
$ cargo install cargo-web
```
## Either start a simple web server
```sh
$ cargo web start
```
## Or generate a static site
```sh
$ cargo web deploy
```
