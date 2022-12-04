# My solutions for Advent of Code 2022

## Goals

My main goal for participating in the Advent of Code for this year is to learn Rust.
Therefore, I set myself some additional conditions:
* No use of external crates (aside those needed for wasm).
* It is possible to re-run the solution of every part of every puzzle.

And some extension goals:
* Do proper error handling.
* Reuse this code on in a web application.

Note: there will be some refactoring going on as Advent of Code progresses, so the final product may be a bit different from my initial solutions.

## Building

To build to wasm, run the following:
```
wasm-pack build --features wasm --release
```
