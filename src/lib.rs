use std::hash::{Hash, Hasher, SipHasher};
//use std::collections::BitVec;

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
    buckets: Vec<bool>, // BitVec,
    hashes: u64,
    item_count: u64,
}

impl BloomFilter {
    pub fn new(size: usize, hashes: u64) -> BloomFilter {
        let buckets = vec![false; size]; //BitVec::from_elem(size, false);

        BloomFilter { buckets: buckets, hashes: hashes, item_count: 0 }
    }

    pub fn insert<T>(&mut self, word: &T) 
        where T:Hash
    {
        for seed in 0..self.hashes {
            let i: usize = self.bloom_hash(&word, seed);

            self.buckets[i] = true;
            //self.buckets.set(i, true);
        }
        self.item_count += 1;
    }

    pub fn check<T>(&mut self, word: &T) -> bool 
        where T: Hash
    {
        for seed in 0..self.hashes {
            let i: usize = self.bloom_hash(&word, seed);

            if !self.buckets[i] {
                return false;
            }
        }

        true
    }

    pub fn error_chance(&mut self) -> f32 {
        let numerator =  (self.hashes * self.item_count) as f32;
        let denominator = self.buckets.len() as f32;
        let e_exponent =  (-1.0 * numerator) / denominator;
        let chance: f32 = (1.0 - e_exponent.exp()).powf(self.hashes as f32);

        chance
    }

    fn bloom_hash<T>(&mut self, word: T, seed: u64) -> usize 
        where T: Hash
    {
        let the_hash: usize = my_hash(&word, seed) as usize;

        the_hash % self.buckets.len()
    }
}

#[test]
fn create_correct_size() {
    let bf = BloomFilter::new(10, 1);
    assert!(bf.buckets.len() == 10);
}

#[test]
fn insert_and_check_str() {
    let mut bf = BloomFilter::new(100, 2);
    bf.insert(&"coffee");

    assert!(bf.check(&"coffee") == true);
    assert!(bf.check(&"pancakes") == false);
}

#[test]
fn insert_and_check_other() {
    let mut bf = BloomFilter::new(100, 2);
    let the_answer = 42;
    let the_devil = 666;

    bf.insert(&the_answer);

    assert!(bf.check(&the_answer) == true);
    assert!(bf.check(&the_devil) == false);
}

#[test]
fn insert_and_increment_item_count() {
    let mut bf = BloomFilter::new(100, 2);
    assert!(bf.item_count == 0);

    bf.insert(&"coffee");

    assert!(bf.item_count == 1);

    bf.insert(&"ham");
    
    assert!(bf.item_count == 2);
}

#[test]
fn error_chance() {
    let mut bf = BloomFilter::new(100, 2);

    bf.insert(&"coffee");

    assert!(bf.error_chance().floor() == 0.0);
}