use std::collections::LinkedList;

const BUCKET_SIZE: i32 = 32;

struct MyHashMap {
    map: Vec<LinkedList<(i32, i32)>>    
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap { map: vec![LinkedList::new(); BUCKET_SIZE as usize] }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let hash = (key % BUCKET_SIZE) as usize;
        
        for (k, v) in self.map[hash].iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }
        
        self.map[hash].push_back((key, value));
    }
    
    fn get(&self, key: i32) -> i32 {
        let hash = (key % BUCKET_SIZE) as usize;
        
        for (k, v) in self.map[hash].iter() {
            if *k == key {
                return *v;
            }
        }
        
        -1
    }
    
    fn remove(&mut self, key: i32) {
        let hash = (key % BUCKET_SIZE) as usize;
        let mut index: usize = 0;
        
        for (k, v) in self.map[hash].iter() {
            if *k == key {
                break;
            }
            index += 1;
        }
        
        if index < self.map[hash].len() {
            let mut rem = self.map[hash].split_off(index);
            rem.pop_front();
            self.map[hash].append(&mut rem);
        }
    }
}
