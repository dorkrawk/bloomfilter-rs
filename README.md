# BloomFilter RS
A Bloom filter written in Rust.

*last tested with Rust 1.1.0*

## Example
For a Bloom filter with 100 buckets and 5 hash functions:
```rust
let mut bf = BloomFilter::new(100, 5);

bf.insert(&"hamster");
bf.insert(&"coffee");

bf.check(&"hamster");
// true

bf.check(&"oatmeal");
// (probably) false

bf.error_chance();
// 0.000007804244
```

# TODO
- [x] multiple hash functions
- [x] error_chance() function
- [x] handle inserting arbitrary types

*note: I mainly built this project to learn a bit about Rust*
