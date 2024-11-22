use core::hash::{Hash, Hasher};
use alloc::vec::Vec;
use libm::floor;

const TABLE_SIZE: usize = 0xFFF;

pub struct HashMap<K: Hash + PartialEq + Clone, V: Clone> {
    pub table: [Vec<(K,V)>;TABLE_SIZE]
}

impl<K: Hash + PartialEq + Clone, V: Clone> HashMap<K, V>  {
    pub fn new() -> Self {
        Self { 
            table: [const { Vec::new() };TABLE_SIZE]
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let hasher = &mut LocalHasher::new();
        key.hash(hasher);
        let hash = hasher.finish() as usize % TABLE_SIZE;
        
        if self.table[hash].len() == 0 {
            self.table[hash].push((key,value));
            return;
        }

        for (_key, _value) in self.table[hash].iter_mut() {
            if *_key == key {
                *_value = value;
                return;
            }
        }
        
        self.table[hash].push((key,value));
    }

    pub fn at(&mut self, key: K) -> Option<V> {
        let hasher = &mut LocalHasher::new();
        key.hash(hasher);
        let hash = hasher.finish() as usize % TABLE_SIZE;

        if self.table[hash].len() == 0 {
            return None;
        }

        for (_key, _value) in self.table[hash].iter_mut(){
            if *_key == key {
                return Some(_value.clone());
            }
        }
        
        None
    }

    pub fn iter(&self) -> HashMapIter<K, V> {
        HashMapIter::<K, V> { map: self, i: 0, j: 0 }
    }
}

pub struct HashMapIter<'a, K: Clone + Hash + PartialEq , V: Clone> {
    map: &'a HashMap<K, V>,
    i: usize,
    j: usize,
}

impl<'a, K: Clone + Hash + PartialEq, V: Clone> HashMapIter<'a, K, V> {}

impl<'a, K: Clone + Hash + PartialEq, V: Clone> Iterator for HashMapIter<'a, K, V> {
    type Item = &'a (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.i..TABLE_SIZE {
            self.i = i;
            for j in self.j..self.map.table[i].len() {
                self.j = j + 1;
                println!("{}, {}", i , j);
                if j + 1 == self.map.table[i].len() {
                    self.i += 1;
                }
                return Some(&self.map.table[i][j]);
            }
        }

       None
    }
}

static A: f64 = 0.61803398875;
// static M: f64 = 2_u32.pow(16) as f64;
static M: f64 = TABLE_SIZE as f64;
pub struct LocalHasher(f64);

impl LocalHasher {
    pub fn new() -> Self {
        Self(0.0)
    }
}

impl Hasher for LocalHasher {
    fn write(&mut self, bytes: &[u8]) {
        bytes.iter().for_each(|it| {
            let k = (*it as f64) + self.0;
            self.0 = ((k * A) % 1.0) * M
        });
    }

    fn finish(&self) -> u64 {
        floor(self.0) as u64
    }
}
