## Benchmark

`cargo bench --tests`
```
test tests::bench_cpp_b62_to_hex  ... bench:     282,134 ns/iter (+/- 22,600)
test tests::bench_cpp_hex_to_b62  ... bench:     306,185 ns/iter (+/- 73,065)
test tests::bench_rust_b62_to_hex ... bench:     626,555 ns/iter (+/- 107,552)
test tests::bench_rust_hex_to_b62 ... bench:   1,186,583 ns/iter (+/- 90,073)
```