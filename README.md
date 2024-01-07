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
├─ big_table  256 µs        │ 304.4 µs      │ 256.2 µs      │ 259.5 µs      │ 100     │ 100
╰─ teams      322.5 ns      │ 356.4 ns      │ 327.8 ns      │ 330.4 ns      │ 100     │ 1600

     Running benches/handlebars.rs (target/release/deps/handlebars-aa0bcf18de12faf1)
Timer precision: 41 ns
handlebars    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  2.856 ms      │ 3.161 ms      │ 2.904 ms      │ 2.912 ms      │ 100     │ 100
╰─ teams      2.04 µs       │ 5.499 µs      │ 2.124 µs      │ 2.178 µs      │ 100     │ 100

     Running benches/html_node.rs (target/release/deps/html_node-ad34e7e13c92e2e4)
Timer precision: 41 ns
html_node     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  1.084 ms      │ 1.276 ms      │ 1.094 ms      │ 1.1 ms        │ 100     │ 100
╰─ teams      1.957 µs      │ 8.833 µs      │ 2.04 µs       │ 2.109 µs      │ 100     │ 100

     Running benches/hyped.rs (target/release/deps/hyped-f89ff1fe5fc5bc18)
Timer precision: 41 ns
hyped         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  367.1 µs      │ 504.1 µs      │ 370.7 µs      │ 376.3 µs      │ 100     │ 100
╰─ teams      1.166 µs      │ 9.874 µs      │ 1.208 µs      │ 1.326 µs      │ 100     │ 100

     Running benches/maud.rs (target/release/deps/maud-5676299fd73a2710)
Timer precision: 41 ns
maud          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  55.41 µs      │ 88.95 µs      │ 63.45 µs      │ 64.1 µs       │ 100     │ 100
╰─ teams      114.9 ns      │ 338.2 ns      │ 117.8 ns      │ 123.5 ns      │ 100     │ 6400

     Running benches/minijinja.rs (target/release/deps/minijinja-ebac6a74b082e911)
Timer precision: 41 ns
minijinja     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  1.457 ms      │ 1.569 ms      │ 1.461 ms      │ 1.467 ms      │ 100     │ 100
╰─ teams      2.332 µs      │ 7.165 µs      │ 2.374 µs      │ 2.465 µs      │ 100     │ 100

     Running benches/tera.rs (target/release/deps/tera-6b5113bea0d569e7)
Timer precision: 41 ns
tera          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  651.8 µs      │ 1.073 ms      │ 655.3 µs      │ 661.9 µs      │ 100     │ 100
╰─ teams      2.749 µs      │ 7.957 µs      │ 2.832 µs      │ 2.897 µs      │ 100     │ 100

     Running benches/tinytemplate.rs (target/release/deps/tinytemplate-b97f518b0dcf8d22)
Timer precision: 41 ns
tinytemplate  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  348.5 µs      │ 438 µs        │ 351.6 µs      │ 356.2 µs      │ 100     │ 100
╰─ teams      1.249 µs      │ 1.395 µs      │ 1.28 µs       │ 1.288 µs      │ 100     │ 400
```

## Running the benches

```sh
cargo bench
```
