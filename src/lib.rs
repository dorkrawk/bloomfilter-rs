use std::hash::{Hash, Hasher, SipHasher};

#[test]
fn create_correct_size() {
  let bf = BloomFilter::new(10, 1);
  assert!(bf.buckets.len() == 10);
}

#[test]
fn insert_and_check() {
  let mut bf = BloomFilter::new(10, 1);
  bf.insert("dave");
  bf.insert("hamster");
  bf.insert("coffee");

  assert!(bf.check("dave") == true);
  assert!(bf.check("hamster") == true);
  assert!(bf.check("coffee") == true);
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

  pub fn insert(&mut self, word: &str) {
    let i:usize = self.bloom_hash(word);

    self.buckets[i] = true;
  }

  pub fn check(&mut self, word: &str) -> bool {
    let i:usize = self.bloom_hash(word);

    if self.buckets[i] {
      return true;
    } else {
      return false;
    }
  }

  fn bloom_hash(&mut self, word: &str) -> usize {
    let the_hash:usize = my_hash(&word) as usize;
    return the_hash % self.buckets.len();
  }
}
