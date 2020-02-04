## Benchmark

`cargo bench --tests`
```
test tests::bench_cpp_b62_to_hex  ... bench:     299,504 ns/iter (+/- 84,396)
test tests::bench_cpp_hex_to_b62  ... bench:     301,077 ns/iter (+/- 72,611)
test tests::bench_rust_b62_to_hex ... bench:     213,183 ns/iter (+/- 25,950)
test tests::bench_rust_hex_to_b62 ... bench:     143,637 ns/iter (+/- 19,035)
```