use std::hash::{Hash, Hasher, SipHasher};

#[test]
fn create_correct_size() {
  let bf = BloomFilter::new(10, 1);
  assert!(bf.buckets.len() == 10);
}

#[test]
fn insert_and_check_str() {
  let mut bf = BloomFilter::new(100, 2);
  bf.insert("coffee");

  assert!(bf.check("coffee") == true);
  assert!(bf.check("pancakes") == false);
}

#[test]
fn insert_and_check_other() {
  let mut bf = BloomFilter::new(100, 2);
  bf.insert(42);

  assert!(bf.check(42) == true);
  assert!(bf.check(666) == false);
}

fn my_hash<T>(obj: T, seed: u64) -> u64
    where T: Hash
{
    let key1 = seed;
    let key2 = seed + 1;
    let mut hasher = SipHasher::new_with_keys(key1, key2);
    obj.hash(&mut hasher);
    hasher.finish()
}

pub struct BloomFilter {
  buckets: Vec<bool>,
  hashes: u64
}

impl BloomFilter {

  pub fn new(size: usize, hashes: u64) -> BloomFilter {
    let buckets = vec![false; size];

    BloomFilter { buckets: buckets, hashes: hashes }
  }

  pub fn insert<T>(&mut self, word: T) 
      where T:Hash
  {
    for seed in 0..self.hashes {
      let i:usize = self.bloom_hash(&word, seed);

      self.buckets[i] = true;
    }
  }

  pub fn check<T>(&mut self, word: T) -> bool 
      where T: Hash
  {
    for seed in 0..self.hashes {
      let i:usize = self.bloom_hash(&word, seed);

      if !self.buckets[i] {
        return false;
      }
    }
    return true;
  }

  fn bloom_hash<T>(&mut self, word: T, seed: u64) -> usize 
      where T: Hash
  {
    let the_hash:usize = my_hash(&word, seed) as usize;
    return the_hash % self.buckets.len();
  }
}
