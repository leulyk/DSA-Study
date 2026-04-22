use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            let mut frequency = map.entry(num).or_insert(0);
            *frequency += 1;

            if *frequency > 1 {
                return true;
            }
        } 

        false
    }
}
