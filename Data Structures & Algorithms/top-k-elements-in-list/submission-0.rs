use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();
        let mut heap = BinaryHeap::new();

        for num in &nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        for (num, freq) in freq {
            heap.push((freq, num));
        }

        let mut top_frequent = vec![];
        for _ in 0..k {
            if let Some((_, &num)) = heap.pop() {
                top_frequent.push(num);
            }
        }

        top_frequent
    }
}
