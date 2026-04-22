impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut aux = vec![0; nums.len()];
        let high = nums.len() - 1;
        Self::merge_sort(&mut nums, &mut aux, 0, high);
        nums
    }

    fn merge_sort(nums: &mut Vec<i32>, aux: &mut Vec<i32>, low: usize, high: usize) {
        if low < high {
            let mid = (low + high) / 2;
            Self::merge_sort(nums, aux, low, mid);
            Self::merge_sort(nums, aux, mid + 1, high);
            Self::merge(nums, aux, low, mid, high);
        }
    }

    fn merge(arr: &mut Vec<i32>, aux: &mut Vec<i32>, low: usize, mid: usize, high: usize) {
        for k in low..high + 1 {
            aux[k] = arr[k];
        }

        let mut i = low;
        let mut j = mid + 1;

        for k in low..high + 1 {
            if i > mid {
                arr[k] = aux[j];
                j += 1;
            } else if j > high {
                arr[k] = aux[i];
                i += 1;
            } else if aux[i] < aux[j] {
                arr[k] = aux[i];
                i += 1;
            } else {
                arr[k] = aux[j];
                j += 1;
            }
        }
    }
}