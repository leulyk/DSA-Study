impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut length = 0;

        for index in 0..nums.len() {
            if nums[index] != val {
                nums[length] = nums[index];
                length += 1;
            }
        }

        length as i32
    }
}