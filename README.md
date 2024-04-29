# Rust html rendering benchmarks

This repo is a resurrection of [template-benchmark-rs](https://github.com/rosetta-rs/template-benchmark-rs) with divan instead of criterion.

These projects are in the running:

- [askama](https://github.com/djc/askama)
- [handlebars](https://github.com/sunng87/handlebars-rust)
- [html_node](https://github.com/vidhanio/html-node)
- [hypertext](https://github.com/vidhanio/hypertext)
- [hyped](https://github.com/swlkr/hyped)
- [maud](https://github.com/lambda-fairy/maud)
- [minijinja](https://github.com/mitsuhiko/minijinja)
- [tera](https://github.com/Keats/tera)
- [tiny template](https://github.com/bheisler/TinyTemplate)

## Results

These results are from rustc 1.79.0-nightly (ccfcd950b 2024-04-15), running on a 2021 Macbook Pro M1 Pro cpu 16GB RAM.

```zsh
     Running benches/askama.rs (target/release/deps/askama-52aa98d72d807683)
Timer precision: 41 ns
askama        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  254.2 µs      │ 282.6 µs      │ 254.7 µs      │ 259.1 µs      │ 100     │ 100
╰─ teams      290.6 ns      │ 6.415 µs      │ 333.6 ns      │ 408.8 ns      │ 100     │ 100

     Running benches/handlebars.rs (target/release/deps/handlebars-c26a2900f909b89f)
Timer precision: 41 ns
handlebars    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  3.351 ms      │ 3.53 ms       │ 3.424 ms      │ 3.421 ms      │ 100     │ 100
╰─ teams      2.165 µs      │ 8.708 µs      │ 2.208 µs      │ 2.301 µs      │ 100     │ 100

     Running benches/html_node.rs (target/release/deps/html_node-2451d678ebdeed24)
Timer precision: 41 ns
html_node     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  1.019 ms      │ 1.292 ms      │ 1.032 ms      │ 1.045 ms      │ 100     │ 100
╰─ teams      1.916 µs      │ 10.62 µs      │ 1.999 µs      │ 2.122 µs      │ 100     │ 100

     Running benches/hyped.rs (target/release/deps/hyped-0e6f6a44402c6f44)
Timer precision: 41 ns
hyped         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  366.6 µs      │ 483 µs        │ 369.7 µs      │ 374.2 µs      │ 100     │ 100
╰─ teams      1.124 µs      │ 8.291 µs      │ 1.208 µs      │ 1.302 µs      │ 100     │ 100

     Running benches/hypertext.rs (target/release/deps/hypertext-af0d1f5f20d0848e)
Timer precision: 41 ns
hypertext     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  38.74 µs      │ 62.04 µs      │ 39.16 µs      │ 40.81 µs      │ 100     │ 100
╰─ teams      105.1 ns      │ 283.5 ns      │ 106.4 ns      │ 111 ns        │ 100     │ 6400

     Running benches/maud.rs (target/release/deps/maud-2708bacebe066da6)
Timer precision: 41 ns
maud          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  51.79 µs      │ 85.08 µs      │ 52.35 µs      │ 54.19 µs      │ 100     │ 100
╰─ teams      92.12 ns      │ 156.5 ns      │ 92.78 ns      │ 93.93 ns      │ 100     │ 6400

     Running benches/minijinja.rs (target/release/deps/minijinja-d553dec2334bdc0f)
Timer precision: 41 ns
minijinja     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  1.703 ms      │ 1.803 ms      │ 1.719 ms      │ 1.727 ms      │ 100     │ 100
╰─ teams      2.207 µs      │ 7.499 µs      │ 2.291 µs      │ 2.378 µs      │ 100     │ 100

     Running benches/tera.rs (target/release/deps/tera-e7d300215608ee7e)
Timer precision: 41 ns
tera          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  686.4 µs      │ 1.218 ms      │ 691.3 µs      │ 699.1 µs      │ 100     │ 100
╰─ teams      2.832 µs      │ 8.583 µs      │ 2.916 µs      │ 2.992 µs      │ 100     │ 100

     Running benches/tinytemplate.rs (target/release/deps/tinytemplate-b39e990de12df98e)
Timer precision: 41 ns
tinytemplate  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  322.7 µs      │ 392.4 µs      │ 325.7 µs      │ 329.7 µs      │ 100     │ 100
╰─ teams      1.228 µs      │ 1.385 µs      │ 1.249 µs      │ 1.26 µs       │ 100     │ 400
```

## Running the benches

```sh
cargo bench
```
