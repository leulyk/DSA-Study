use std::collections::LinkedList;

const BUCKET_SIZE: i32 = 1024;

struct MyHashSet {
    set: Vec<LinkedList<i32>>
}

impl MyHashSet {

    fn new() -> Self {
        Self {
            set: vec![LinkedList::new(); BUCKET_SIZE as usize]
        } 
    }
    
    fn add(&mut self, key: i32) {
        let hash = (key % BUCKET_SIZE) as usize;

        for k in self.set[hash].iter() {
            if *k == key {
                return;
            }
        }

        self.set[hash].push_back(key);
    }
    
    fn remove(&mut self, key: i32) {
        let hash = (key % BUCKET_SIZE) as usize;
        let mut index: usize = 0;

        for k in self.set[hash].iter() {
            if *k == key {
                break;
            }
            index += 1;
        }
        
        if index < self.set[hash].len() {
            let mut rem = self.set[hash].split_off(index);
            rem.pop_front();
            self.set[hash].append(&mut rem);
        }
    }
    
    fn contains(&self, key: i32) -> bool {
        let hash = (key % BUCKET_SIZE) as usize;

        for k in self.set[hash].iter() {
            if *k == key {
                return true;
            }
        }
        
        false
    }
}
