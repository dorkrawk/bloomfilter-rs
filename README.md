# BloomFilter RS
A Bloom filter written in Rust.

## Example
```
let mut bf = BloomFilter::new(10, 1);

bf.insert("hamster");
bf.insert("coffee");

bf.check("hamster");
// true

bf.check("oatmeal");
// (probably) false
```

# TODO
- [ ] multiple hash functions
- [ ] error_chance() function
- [x] handle inserting arbitrary types

*note: I mainly built this project to learn a bit about Rust*