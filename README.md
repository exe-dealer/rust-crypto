# Rust-MD5

[![Build Status](https://travis-ci.org/exe-dealer/rust-md5.svg)](https://travis-ci.org/exe-dealer/rust-md5)

A pure-Rust implementation of MD5 algorithm.

## Usage

To use Rust-MD5, add the following to your Cargo.toml:

```toml
[dependencies.rust-md5]
git = "https://github.com/exe-dealer/rust-md5.git"
```

and the following to your crate root:

```rust
extern crate md5;

let hasher = Md5::new();
hasher.input_str("somestr");
println!("{}", hasher.result_str());
```
