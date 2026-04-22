impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let high = nums.len() - 1;
        Self::quick_sort(&mut nums, 0, high);
        nums
    }

    fn quick_sort(nums: &mut Vec<i32>, low: usize, high: usize) {
        if low >= high || low < 0 || high >= nums.len() {
            return;
        }

        let pivot = Self::partition(nums, low, high);

        Self::quick_sort(nums, low, pivot);
        Self::quick_sort(nums, pivot + 1, high);
    }

    fn partition(nums: &mut Vec<i32>, low: usize, high: usize) -> usize {
        let mut pivot = low;

        let mut i = low - 1;
        let mut j = high + 1;

        loop {
            loop {
                i += 1;
                if nums[i] >= nums[pivot] {
                    break;
                }
            } 

            loop {
                j -= 1;
                if nums[j] <= nums[pivot] {
                    break;
                }
            }

            if i >= j {
                return j;
            }

            let temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
        }
    }
}

