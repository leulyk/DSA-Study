use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for num in &nums {
            set.insert(num);
        }

        let mut long_seq = 0;

        for num in &nums {
            if set.get(&(num - 1)).is_none() {
                let mut seq_count = 1;
                let mut temp = num + 1;

                while set.get(&temp).is_some() {
                    seq_count += 1;
                    temp += 1;
                }

                if seq_count > long_seq {
                    long_seq = seq_count;
                }
            }
        }

        long_seq
    }
}
