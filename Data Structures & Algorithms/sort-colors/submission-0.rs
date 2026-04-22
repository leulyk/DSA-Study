impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut red, mut white, mut blue) = (0, 0, 0);

        for i in 0..nums.len() {
            match nums[i] {
                0 => { red += 1; },
                1 => { white += 1; },
                2 => { blue += 1; }
                _ => ()
            }
        }

        for i in 0..nums.len() {
            nums[i] = if red > 0 {
                red -= 1;
                0
            } else if white > 0 {
                white -= 1;
                1
            } else {
                blue -= 1;
                2
            }
        }
    }
}