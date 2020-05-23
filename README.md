# Learning to code in the Rust Programming Language

## 1 - Getting Started

* Created a `hello-world` program and ran it using `rustc`
* Created and built a `hello-cargo` project using `cargo new` and `cargo run`

Important command: `cargo build --release`

## 2 - Programming a guessing game

* Made a game to take `user input` and guess the random number generated using `rand` crate

Important command: `cargo doc --open`

## 3 - Common Programming Concepts

* `isize` and `usize` depend on computer architecure (64 bits for 64-bit-architecture and 32 bits for 32-bit-architecture)
* `loop` (not `while` or `for`) and `if/else` are expressions, not statements, meaning they can return values

## 4 - Ownership
* Ownership means the `Power to destroy (Drop)`
* Ownership is important only for types stored in `heap` because `stack` doesn't have the issue of allocation and deallocation of memory in runtime
* A variable can have only one owner at a time