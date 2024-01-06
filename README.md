# Rust html rendering benchmarks

This repo is a resurrection of [template-benchmark-rs](https://github.com/rosetta-rs/template-benchmark-rs) with divan instead of criterion.

These three projects are in the running:

- [askama](https://github.com/djc/askama)
- [hyped](https://github.com/swlkr/hyped)
- [maud](https://github.com/lambda-fairy/maud)
- [tera](https://github.com/Keats/tera)
- [tiny template](https://github.com/bheisler/TinyTemplate)

## Results

These results are from 2024-01-06 (rustc 1.75.0), running on a Mac Mini (2020) M1 cpu.

```zsh
     Running benches/askama.rs (target/release/deps/askama-f4ff65d9ab80e3e7)
Timer precision: 41 ns
askama        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  256 µs        │ 318.4 µs      │ 256.4 µs      │ 260.1 µs      │ 100     │ 100
╰─ teams      312.1 ns      │ 361.6 ns      │ 322.5 ns      │ 324.1 ns      │ 100     │ 1600

     Running benches/hyped.rs (target/release/deps/hyped-90b08ee0239cb36a)
Timer precision: 41 ns
hyped         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  366.4 µs      │ 459.1 µs      │ 367.4 µs      │ 372.2 µs      │ 100     │ 100
╰─ teams      1.165 µs      │ 9.082 µs      │ 1.229 µs      │ 1.318 µs      │ 100     │ 100

     Running benches/maud.rs (target/release/deps/maud-0df41b278f0a9572)
Timer precision: 41 ns
maud          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  62.41 µs      │ 94.16 µs      │ 63.43 µs      │ 64.85 µs      │ 100     │ 100
╰─ teams      119.4 ns      │ 144.2 ns      │ 129.9 ns      │ 129.9 ns      │ 100     │ 3200

     Running benches/tera.rs (target/release/deps/tera-81168af8e2c649a3)
Timer precision: 41 ns
tera          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  662.8 µs      │ 2.184 ms      │ 667.9 µs      │ 684.7 µs      │ 100     │ 100
╰─ teams      2.79 µs       │ 7.666 µs      │ 2.874 µs      │ 2.933 µs      │ 100     │ 100
```

## Running the benches

```sh
cargo bench
```
