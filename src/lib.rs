use std::hash::{Hash, Hasher, SipHasher};

#[test]
fn create_correct_size() {
  let bf = BloomFilter::new(10, 1);
  assert!(bf.buckets.len() == 10);
}

#[test]
fn insert_and_check_str() {
  let mut bf = BloomFilter::new(10, 1);
  bf.insert("coffee");

  assert!(bf.check("coffee") == true);
  assert!(bf.check("pancakes") == false);
}

#[test]
fn insert_and_check_other() {
  let mut bf = BloomFilter::new(10, 1);
  bf.insert(42);

  assert!(bf.check(42) == true);
  assert!(bf.check(666) == false);
}

fn my_hash<T>(obj: T) -> u64
    where T: Hash
{
    let mut hasher = SipHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

pub struct BloomFilter {
  buckets: Vec<bool>,
  hashes: u32
}

impl BloomFilter {

  pub fn new(size: usize, hashes: u32) -> BloomFilter {
    let buckets = vec![false; size];

    BloomFilter { buckets: buckets, hashes: hashes }
  }

  pub fn insert<T>(&mut self, word: T) 
      where T:Hash
  {
    let i:usize = self.bloom_hash(word);

    self.buckets[i] = true;
  }

  pub fn check<T>(&mut self, word: T) -> bool 
      where T: Hash
  {
    let i:usize = self.bloom_hash(word);

    if self.buckets[i] {
      return true;
    } else {
      return false;
    }
  }

  fn bloom_hash<T>(&mut self, word: T) -> usize 
      where T: Hash
  {
    let the_hash:usize = my_hash(&word) as usize;
    return the_hash % self.buckets.len();
  }
}
