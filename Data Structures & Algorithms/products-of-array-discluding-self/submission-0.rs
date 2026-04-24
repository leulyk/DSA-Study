impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len() + 1;
        let mut prefix_product = vec![1; length];
        let mut suffix_product = vec![1; length];

        for i in 1..length {
            prefix_product[i] = prefix_product[i - 1] * nums[i - 1];
            suffix_product[length - i - 1] = suffix_product[length - i] * nums[length - i - 1];
        }

        let mut products = vec![1; nums.len()];
        for i in 0..nums.len() {
            products[i] = prefix_product[i] * suffix_product[i + 1];
        }

        products
    }
}
