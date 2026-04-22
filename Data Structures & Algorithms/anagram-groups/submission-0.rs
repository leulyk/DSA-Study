use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort_unstable();

            let sorted_str: String = chars.into_iter().collect();
            let mut values = map.entry(sorted_str).or_insert(vec![]);
            values.push(str);
        }    

        map.into_values().collect()
    }
}
