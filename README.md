# Rust html rendering benchmarks

This repo is a resurrection of [template-benchmark-rs](https://github.com/rosetta-rs/template-benchmark-rs) with divan instead of criterion.

These three projects are in the running:

- [askama](https://github.com/djc/askama)
- [hyped](https://github.com/swlkr/hyped)
- [maud](https://github.com/lambda-fairy/maud)

## Results

These results are from 2024-01-06 (rustc 1.75.0), running on a Mac Mini (2020) M1 cpu.

```zsh
     Running benches/askama.rs (target/release/deps/askama-35ed2ee0848ebc54)
Timer precision: 41 ns
askama        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  256.1 µs      │ 301.4 µs      │ 265.3 µs      │ 267.4 µs      │ 100     │ 100
╰─ teams      322.5 ns      │ 661.1 ns      │ 330.4 ns      │ 336.3 ns      │ 100     │ 1600

     Running benches/hyped.rs (target/release/deps/hyped-599fad67cb1a6bc1)
Timer precision: 41 ns
hyped         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  366.4 µs      │ 444.2 µs      │ 367.8 µs      │ 372.7 µs      │ 100     │ 100
╰─ teams      1.165 µs      │ 9.207 µs      │ 1.208 µs      │ 1.313 µs      │ 100     │ 100

     Running benches/maud.rs (target/release/deps/maud-d78a3e58c1da5b65)
Timer precision: 41 ns
maud          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  56.62 µs      │ 82.24 µs      │ 63.54 µs      │ 61.99 µs      │ 100     │ 100
╰─ teams      115.5 ns      │ 137 ns        │ 122.7 ns      │ 122.3 ns      │ 100     │ 6400
```

## Running the benches

```sh
cargo bench
```
