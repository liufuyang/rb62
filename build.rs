extern crate cc;

fn main() {
    // println!("cargo:rustc-link-search=all=src/cpp");
    // println!("cargo:rustc-link-lib=dylib=base62_conversion.o");

    #[cfg(feature = "bench_cpp")]
    cc::Build::new()
        .cpp(true)
        .file("src/cpp/base62_conversion.cpp")
        .compile("libcppb62.a");
}