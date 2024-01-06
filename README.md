# Rust html rendering benchmarks

This repo is a resurrection of [template-benchmark-rs](https://github.com/rosetta-rs/template-benchmark-rs) with divan instead of criterion.

These projects are in the running:

- [askama](https://github.com/djc/askama)
- [hyped](https://github.com/swlkr/hyped)
- [maud](https://github.com/lambda-fairy/maud)
- [tera](https://github.com/Keats/tera)
- [tiny template](https://github.com/bheisler/TinyTemplate)

## Results

These results are from 2024-01-06 (rustc 1.75.0), running on a Mac Mini (2020) M1 cpu.

```zsh
     Running benches/askama.rs (target/release/deps/askama-bb6ec71bceda2ebb)
Timer precision: 41 ns
askama        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  256.1 µs      │ 294.5 µs      │ 256.5 µs      │ 259.6 µs      │ 100     │ 100
╰─ teams      312.1 ns      │ 601.2 ns      │ 317.4 ns      │ 327.7 ns      │ 100     │ 1600

     Running benches/hyped.rs (target/release/deps/hyped-c926b7725d86d59d)
Timer precision: 41 ns
hyped         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  365.7 µs      │ 497.6 µs      │ 367.6 µs      │ 374.1 µs      │ 100     │ 100
╰─ teams      1.124 µs      │ 7.832 µs      │ 1.207 µs      │ 1.294 µs      │ 100     │ 100

     Running benches/maud.rs (target/release/deps/maud-9ae697d4a3a8a35b)
Timer precision: 41 ns
maud          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  62.41 µs      │ 79.99 µs      │ 63.79 µs      │ 64.48 µs      │ 100     │ 100
╰─ teams      118.8 ns      │ 132.4 ns      │ 126.6 ns      │ 125.9 ns      │ 100     │ 6400

     Running benches/tera.rs (target/release/deps/tera-4188ee14966ba4dd)
Timer precision: 41 ns
tera          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  662.7 µs      │ 4.754 ms      │ 670.5 µs      │ 716.6 µs      │ 100     │ 100
╰─ teams      2.79 µs       │ 18.41 µs      │ 2.874 µs      │ 3.03 µs       │ 100     │ 100

     Running benches/tinytemplate.rs (target/release/deps/tinytemplate-a8aba47907921de2)
Timer precision: 41 ns
tinytemplate  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_table  467.7 µs      │ 1.069 ms      │ 648.2 µs      │ 657.9 µs      │ 100     │ 100
╰─ teams      1.624 µs      │ 4.583 µs      │ 1.624 µs      │ 1.69 µs       │ 100     │ 100
```

## Running the benches

```sh
cargo bench
```
