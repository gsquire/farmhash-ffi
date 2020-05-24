# FarmHash FFI
[![CI](https://github.com/gsquire/farmhash-ffi/workflows/CI/badge.svg)](https://github.com/gsquire/farmhash-ffi/actions)
[![docs](https://docs.rs/farmhash-ffi/badge.svg)](https://docs.rs/farmhash-ffi)

This crate provides a simple API to utilize some of the FarmHash hashing functions in
Rust. The types implement the
[`Hasher`](https://doc.rust-lang.org/stable/std/hash/trait.Hasher.html) trait so they can be used
as the hashing function for a
[`HashMap`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html).

## C++ Implementation
The included code can be viewed [here](https://github.com/google/farmhash).

## License
MIT

For the included C++ code, the MIT license can be viewed in [COPYING](include/COPYING).
