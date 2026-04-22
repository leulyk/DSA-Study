impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();

        for i in 0..length {
            let val = nums[i];
            nums.push(val); 
        }

        nums
    }
}
