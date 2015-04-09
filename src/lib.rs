use std::hash::{hash, Hash, SipHasher};

#[test]
fn hello() {
  println!("Hello, bloomfilter-rs!");
}

#[test]
fn filter_size() {
  let filter = BloomFilter::new(10, 2);
}

pub struct BloomFilter {
  buckets: Vec<bool>,
  hashes: u32
}

impl BloomFilter {

  pub fn new(size: usize, hashes: u32) -> BloomFilter {
    let mut buckets = vec![false; size];

    BloomFilter { buckets: buckets, hashes: hashes }
  }
}
