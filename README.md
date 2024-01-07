# Rust html rendering benchmarks

This repo is a resurrection of [template-benchmark-rs](https://github.com/rosetta-rs/template-benchmark-rs) with divan instead of criterion.

These projects are in the running:

- [askama](https://github.com/djc/askama)
- [hyped](https://github.com/swlkr/hyped)
- [maud](https://github.com/lambda-fairy/maud)
- [tera](https://github.com/Keats/tera)
- [tiny template](https://github.com/bheisler/TinyTemplate)
- [handlebars](https://github.com/sunng87/handlebars-rust)

## Results

These results are from 2024-01-07 (rustc 1.75.0), running on a Mac Mini (2020) M1 cpu.

```zsh
     Running benches/askama.rs (target/release/deps/askama-b2de23f0c8275577)
Timer precision: 41 ns
askama        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  256.1 µs      │ 291.3 µs      │ 256.4 µs      │ 259.3 µs      │ 100     │ 100
╰─ teams      314.8 ns      │ 356.4 ns      │ 327.8 ns      │ 331.7 ns      │ 100     │ 1600

     Running benches/handlebars.rs (target/release/deps/handlebars-9af7a2e0ba244318)
Timer precision: 41 ns
handlebars    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  2.915 ms      │ 3.036 ms      │ 2.928 ms      │ 2.929 ms      │ 100     │ 100
╰─ teams      2.041 µs      │ 6.583 µs      │ 2.124 µs      │ 2.196 µs      │ 100     │ 100

     Running benches/hyped.rs (target/release/deps/hyped-6dfe81227b1dbfcb)
Timer precision: 41 ns
hyped         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  367.7 µs      │ 478.5 µs      │ 370.2 µs      │ 373.4 µs      │ 100     │ 100
╰─ teams      1.165 µs      │ 9.457 µs      │ 1.208 µs      │ 1.323 µs      │ 100     │ 100

     Running benches/maud.rs (target/release/deps/maud-98252ae73bcd1f07)
Timer precision: 41 ns
maud          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  62.79 µs      │ 77.62 µs      │ 63.12 µs      │ 63.4 µs       │ 100     │ 100
╰─ teams      116.8 ns      │ 135.1 ns      │ 124 ns        │ 124.5 ns      │ 100     │ 6400

     Running benches/minijinja.rs (target/release/deps/minijinja-a1b3ad120ae16ecc)
Timer precision: 41 ns
minijinja     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  1.459 ms      │ 1.507 ms      │ 1.461 ms      │ 1.462 ms      │ 100     │ 100
╰─ teams      2.249 µs      │ 7.916 µs      │ 2.333 µs      │ 2.433 µs      │ 100     │ 100

     Running benches/tera.rs (target/release/deps/tera-a9b828cc5c61afc4)
Timer precision: 41 ns
tera          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  667.5 µs      │ 1.073 ms      │ 670.8 µs      │ 676.4 µs      │ 100     │ 100
╰─ teams      2.791 µs      │ 8.374 µs      │ 2.915 µs      │ 2.976 µs      │ 100     │ 100

     Running benches/tinytemplate.rs (target/release/deps/tinytemplate-07c2163033e33ce5)
Timer precision: 41 ns
tinytemplate  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  348.2 µs      │ 418.2 µs      │ 351.2 µs      │ 353.7 µs      │ 100     │ 100
╰─ teams      1.249 µs      │ 1.385 µs      │ 1.28 µs       │ 1.297 µs      │ 100     │ 400
```

## Running the benches

```sh
cargo bench
```
