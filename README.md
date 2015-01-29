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
```

```rust
let mut hasher = Md5::new();
hasher.input_str("The quick brown fox jumps over the lazy dog");
let output = &hasher.result_str()[];
assert_eq!(output, "9e107d9d372bb6826bd81d3542a419d6");
```
