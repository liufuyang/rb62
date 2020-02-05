
This is a somehow copy and simpler version of the C++ code from client-core
at https://ghe.spotify.net/spotify-sdk/client-core/blob/master/spotify/libs/tl/cpp/src/base62_conversion.cpp

The code here has been fully tested by migrate all the test cases from 
https://ghe.spotify.net/spotify-sdk/client-core/tree/master/spotify/libs/tl/cpp/tests/detail

This crate support `no_std` environment by default.

## Usage

Firstly clone this repo
```
git clone git@ghe.spotify.net:fuyangl/rb62.git
```

Then add this to your Cargo.toml
```
# Cargo.toml
[dependencies]
rb62 = { path = '../rb62' }
```

Then you can using it like this (or clone the repo and do `cargo run --example demo`)
```rust
use rb62;
use std::str;

fn main() {
    let b62 = "6GGODyP2LIdbxIfYxy5UbN";
    let hex_as_u128 = rb62::get_integer(b62).unwrap();
    let hex = format!("{:032x}", hex_as_u128);
    println!("Input b62 {}, output hex {}", b62, hex);

    let hex = "dbc3d5ebe344484da3e2448712a02213";
    let b62 = rb62::get_b62(hex).unwrap();
    println!("Input hex {}, output b62 {:?}", hex, str::from_utf8(&b62).unwrap());
}
```
gives output:
```
Input b62 6GGODyP2LIdbxIfYxy5UbN, output hex dbc3d5ebe344484da3e2448712a02213
Input hex dbc3d5ebe344484da3e2448712a02213, output b62 6GGODyP2LIdbxIfYxy5UbN
```

## Benchmark

`cargo bench --tests`
```
test bench_cpp_b62_to_hex                   ... bench:     296,346 ns/iter (+/- 34,520)
test bench_cpp_hex_to_b62                   ... bench:     310,960 ns/iter (+/- 22,825)
test bench_rust_b62_to_hex                  ... bench:     218,408 ns/iter (+/- 51,229)
test bench_rust_hex_to_b62                  ... bench:     112,212 ns/iter (+/- 9,260)
test bench_single_operation_cpp_b62_to_hex  ... bench:         436 ns/iter (+/- 31)
test bench_single_operation_cpp_hex_to_b62  ... bench:         506 ns/iter (+/- 134)
test bench_single_operation_rust_b62_to_hex ... bench:         155 ns/iter (+/- 25)
test bench_single_operation_rust_hex_to_b62 ... bench:         196 ns/iter (+/- 59)
```