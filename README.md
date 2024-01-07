# Rust html rendering benchmarks

This repo is a resurrection of [template-benchmark-rs](https://github.com/rosetta-rs/template-benchmark-rs) with divan instead of criterion.

These projects are in the running:

- [askama](https://github.com/djc/askama)
- [handlebars](https://github.com/sunng87/handlebars-rust)
- [html_node](https://github.com/vidhanio/html-node)
- [hyped](https://github.com/swlkr/hyped)
- [maud](https://github.com/lambda-fairy/maud)
- [minijinja](https://github.com/mitsuhiko/minijinja)
- [tera](https://github.com/Keats/tera)
- [tiny template](https://github.com/bheisler/TinyTemplate)

## Results

These results are from 2024-01-07 (rustc 1.75.0), running on a Mac Mini (2020) M1 cpu.

```zsh
     Running benches/askama.rs (target/release/deps/askama-dfd3f05c653d84e7)
Timer precision: 41 ns
askama        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  256.1 µs      │ 293.4 µs      │ 256.5 µs      │ 259.5 µs      │ 100     │ 100
╰─ teams      325.1 ns      │ 353.8 ns      │ 330.4 ns      │ 331.4 ns      │ 100     │ 1600

     Running benches/handlebars.rs (target/release/deps/handlebars-aa0bcf18de12faf1)
Timer precision: 41 ns
handlebars    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  2.862 ms      │ 2.994 ms      │ 2.892 ms      │ 2.896 ms      │ 100     │ 100
╰─ teams      2.082 µs      │ 6.207 µs      │ 2.166 µs      │ 2.222 µs      │ 100     │ 100

     Running benches/html_node.rs (target/release/deps/html_node-ad34e7e13c92e2e4)
Timer precision: 41 ns
html_node     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  1.085 ms      │ 1.339 ms      │ 1.094 ms      │ 1.101 ms      │ 100     │ 100
╰─ teams      1.957 µs      │ 8.499 µs      │ 1.999 µs      │ 2.097 µs      │ 100     │ 100

     Running benches/hyped.rs (target/release/deps/hyped-f89ff1fe5fc5bc18)
Timer precision: 41 ns
hyped         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  366.8 µs      │ 541.9 µs      │ 370.7 µs      │ 376.4 µs      │ 100     │ 100
╰─ teams      1.165 µs      │ 9.749 µs      │ 1.208 µs      │ 1.323 µs      │ 100     │ 100

     Running benches/maud.rs (target/release/deps/maud-5676299fd73a2710)
Timer precision: 41 ns
maud          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  62.54 µs      │ 84.99 µs      │ 63.7 µs       │ 64.86 µs      │ 100     │ 100
╰─ teams      119.4 ns      │ 144.2 ns      │ 125.3 ns      │ 125.7 ns      │ 100     │ 6400

     Running benches/minijinja.rs (target/release/deps/minijinja-ebac6a74b082e911)
Timer precision: 41 ns
minijinja     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  1.455 ms      │ 1.522 ms      │ 1.459 ms      │ 1.466 ms      │ 100     │ 100
╰─ teams      2.291 µs      │ 7.583 µs      │ 2.374 µs      │ 2.462 µs      │ 100     │ 100

     Running benches/tera.rs (target/release/deps/tera-6b5113bea0d569e7)
Timer precision: 41 ns
tera          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  654 µs        │ 1.255 ms      │ 657.8 µs      │ 667 µs        │ 100     │ 100
╰─ teams      2.749 µs      │ 8.54 µs       │ 2.833 µs      │ 2.928 µs      │ 100     │ 100

     Running benches/tinytemplate.rs (target/release/deps/tinytemplate-b97f518b0dcf8d22)
Timer precision: 41 ns
tinytemplate  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  348.4 µs      │ 434.4 µs      │ 351.5 µs      │ 356.5 µs      │ 100     │ 100
╰─ teams      1.291 µs      │ 1.457 µs      │ 1.312 µs      │ 1.322 µs      │ 100     │ 400
```

## Running the benches

```sh
cargo bench
```
