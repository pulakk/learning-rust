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

## 5 - Structs
* `()` is the `unit` type, it's single zero-sized value
* Rust does *automatic referencing and dereferencing*. It doesn't have an equivalent `->` operator. When we call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so object matches the signature of the method. In other words, `p1.distance(&p2);` and `(&p1).distance(&p2);` are the same.
* `::` syntax is used for both `associated functions` and `namespaces` created by modules
* `impl` can be used to implement `methods` or namespaced `associated functions` for structs

## 6 - Enum and Match
* `enum` can be used to enumerate as well as store different forms of data in each enumeration `variant`
* `Option<T>` enum is helpful to handle `None` values and its shortcomings, it has two `variant`s: `Some(T)` and `None`
* `match` or `if let` help handle different scenarios for different `variant`s of an `enum`
* the `_` placeholder implies `else` in a `match`

## 7 - Packages, Crates and Modules
* `idiomatic` `use` should be preferred, e.g. `use std::io;` and then `io::Result` preferred over `use std::io::Result` as the latter might lead to conflicting names
* Bringing structs, enums etc with use is `idiomatic` because they are already namespaced, e.g. `use std::collections::HashMap;` and then `HashMap::new()`
* `as` can be used for aliasing imports, e.g. `use std::io::Result as IoResult`
* use nested paths for inline imports of crates `use std::{cmp::Ordering, io};` or `use std::io:{self, Write};`
* Glob operator is sometimes helpful `std::collections::*;`

## 8 - Common collections
* `Vec`:
    * Multiple data-types can be used in vectors by using an `enum` that has different data type variants
* `String`:
    * Indexing on `String` is not allowed in Rust. We have to take slices.
    *  The slice we take cannot be partial. E.g. `नमस्ते` is of 18 bytes, with 6 characters `न, म, स, ्, त, े, `, so `&word[0..1]` is not allowed as it is just the first byte of the 3 byte character `न`. We can get `न` instead, using `&word[0..3]`. 
    * Different strings may take different bytes per character. `Hola` is of 4 bytes and each character takes up only a single byte.
    * Use `s.split_whitespace()` to iterate over words in a sentence and use `s.chars()` to iterate over every character.
* `HashMap`:
    * Bring into scope using `use std::collections::HashMap;`
    * Create using `HashMap::new()`
    * insert and update using `map.insert(10, 20)`
    * set default value for new key `map.entry(String::from("key")).or_insert(String::from("value"))`, returns a mutable reference which can be used to update if entry already exists or a new one is created
    * will move values for Heap datatypes like String
