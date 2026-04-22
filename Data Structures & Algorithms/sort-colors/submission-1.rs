impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut pivot = 0;
        let mut last_red_index = -1;
        let mut first_blue_index = nums.len();

        while pivot < first_blue_index {
            match nums[pivot] {
                0 => {
                    last_red_index += 1;
                    nums.swap(pivot, last_red_index as usize);
                    pivot += 1;
                },
                1 => {
                    pivot += 1;
                },
                2 => {
                    first_blue_index -= 1;
                    nums.swap(pivot, first_blue_index);
                },
                _ => ()
            }
        }
    }
}