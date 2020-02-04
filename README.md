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

Then you can using it like this
```rust
extern crate rb62;

use rb62::RB62;

fn main() {
    let mut rb62 = RB62::new();
    let b62 = "6GGODyP2LIdbxIfYxy5UbN";
    let hex_as_u128 = rb62.get_integer(b62).unwrap();
    println!("Input b62 {}, output hex {:032x}", b62, hex_as_u128);

    let hex = "dbc3d5ebe344484da3e2448712a02213";
    let b62 = rb62.get_b62(hex).unwrap();
    println!("Input hex {}, output b62 {}", hex, b62);
}
```

## Benchmark

`cargo bench --tests`
```
test tests::bench_cpp_b62_to_hex  ... bench:     299,504 ns/iter (+/- 84,396)
test tests::bench_cpp_hex_to_b62  ... bench:     301,077 ns/iter (+/- 72,611)
test tests::bench_rust_b62_to_hex ... bench:     213,183 ns/iter (+/- 25,950)
test tests::bench_rust_hex_to_b62 ... bench:     143,637 ns/iter (+/- 19,035)
```