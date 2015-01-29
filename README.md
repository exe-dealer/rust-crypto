**NOTE**: The crate name used by rust-crypto has recently changed from `rust-crypto` to
just `crypto`. Please see the Usage section if you are running into issues due to this
change.

# Rust-MD5

[![Build Status](https://travis-ci.org/DaGenix/rust-crypto.png?branch=master)](https://travis-ci.org/DaGenix/rust-crypto)

A pure-Rust implementation of MD5 algorithm.

Rust-Crypto seeks to create practical, auditable, pure-Rust implementations of common cryptographic algorithms
with a minimum amount of assembly code where appropriate. Rust-Crypto supports both x86 and
ARM architectures, although the x86 architecture receives considerably more testing. Rust-Crypto has not been thoroughly
audited for correctness, so any use where security is important is not recommended at this time.

## Usage

To use Rust-Crypto, add the following to your Cargo.toml:

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

