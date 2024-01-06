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

These results are from 2024-01-06 (rustc 1.75.0), running on a Mac Mini (2020) M1 cpu.

```zsh
     Running benches/askama.rs (target/release/deps/askama-2bdf24e68d4a5b26)
Timer precision: 41 ns
askama        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  256.1 µs      │ 284.3 µs      │ 256.5 µs      │ 258.9 µs      │ 100     │ 100
╰─ teams      322.5 ns      │ 353.8 ns      │ 329 ns        │ 330.8 ns      │ 100     │ 1600

     Running benches/handlebars.rs (target/release/deps/handlebars-44967584a81a9efb)
Timer precision: 41 ns
handlebars    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  2.911 ms      │ 3.014 ms      │ 2.94 ms       │ 2.935 ms      │ 100     │ 100
╰─ teams      2.041 µs      │ 5.874 µs      │ 2.124 µs      │ 2.19 µs       │ 100     │ 100

     Running benches/hyped.rs (target/release/deps/hyped-3257526370e8e1eb)
Timer precision: 41 ns
hyped         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  366.8 µs      │ 504.9 µs      │ 370.5 µs      │ 374.9 µs      │ 100     │ 100
╰─ teams      1.165 µs      │ 7.999 µs      │ 1.207 µs      │ 1.284 µs      │ 100     │ 100

     Running benches/maud.rs (target/release/deps/maud-117c8b86413d6514)
Timer precision: 41 ns
maud          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  56.58 µs      │ 88.7 µs       │ 63.87 µs      │ 64.74 µs      │ 100     │ 100
╰─ teams      114.9 ns      │ 146.8 ns      │ 119.4 ns      │ 121.6 ns      │ 100     │ 6400

     Running benches/tera.rs (target/release/deps/tera-8c89bd80be6a95b6)
Timer precision: 41 ns
tera          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  663.2 µs      │ 1.293 ms      │ 668.6 µs      │ 678.5 µs      │ 100     │ 100
╰─ teams      2.749 µs      │ 8.124 µs      │ 2.874 µs      │ 2.93 µs       │ 100     │ 100

     Running benches/tinytemplate.rs (target/release/deps/tinytemplate-b5263d756f70f762)
Timer precision: 41 ns
tinytemplate  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  348.2 µs      │ 448.6 µs      │ 350.5 µs      │ 355 µs        │ 100     │ 100
╰─ teams      1.239 µs      │ 1.364 µs      │ 1.27 µs       │ 1.278 µs      │ 100     │ 400
```

## Running the benches

```sh
cargo bench
```
