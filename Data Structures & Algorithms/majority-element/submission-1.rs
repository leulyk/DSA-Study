impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut element = 0;
        let mut count = 0;

        for &num in nums.iter() {
            if count == 0 {
                element = num;
                count += 1;
            } else if num == element {
                count += 1;
            } else {
                count -= 1;
            }
        }

        element
    }
}
