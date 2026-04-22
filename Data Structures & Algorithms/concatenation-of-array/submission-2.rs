impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut concatenated = Vec::with_capacity(nums.len() * 2);

        for i in 0..(nums.len() * 2) {
            concatenated.push(nums[i % nums.len()]);
        }

        concatenated
    }
}
