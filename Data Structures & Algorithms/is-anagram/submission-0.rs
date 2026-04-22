use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();        

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            if let Some(freq) = map.get_mut(&c) {
                match *freq {
                    0 => return false,
                    1 => { map.remove(&c); },
                    _ => { *freq -= 1; }
                };
            } else {
                return false;
            }
        }

        map.len() == 0
    }
}
