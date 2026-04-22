
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut first_count = 0;
        let mut second_count = 0;
        let mut first_elem = None;
        let mut second_elem = None;

        for &num in &nums {
            if let Some(x) = second_elem && x == num {
                second_count += 1;
            } else if let Some(x) = first_elem && x == num {
                first_count += 1;
            } else if first_count == 0 {
                first_elem = Some(num);
                first_count += 1;
            } else if second_count == 0 {
                second_elem = Some(num);
                second_count += 1;
            } else {
                first_count -= 1;
                second_count -= 1;
            }
        }

        let mut result = vec![];
        first_count = 0;
        second_count = 0;

        for &num in &nums {
            if let Some(x) = second_elem && x == num {
                second_count += 1;
            } else if let Some(x) = first_elem && x == num {
                first_count += 1;
            }
        }

        if first_count > nums.len() / 3 {
            result.push(first_elem.unwrap());
        }
        if second_count > nums.len() / 3 {
            result.push(second_elem.unwrap());
        }

        result
    }
}
