impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut concatenated = vec![0; nums.len() * 2];
        let length = nums.len();

        for i in 0..length {
            concatenated[i] = nums[i];
            concatenated[i + length] = nums[i];
        }

        concatenated
    }
}
