use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            map.insert(num, index);
        }
        
        let mut result: Vec<i32> = vec![];

        for (curr_index, &num) in nums.iter().enumerate() {
            if let Some(index) = map.get(&(target - num)).copied() {
                if index != curr_index {
                    result = vec![index as i32, curr_index as i32];
                    break;
                }
            }
        }
        result.sort();

        result
    }
}