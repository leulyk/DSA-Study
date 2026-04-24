impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let mut products = vec![1; length];

        let mut prefix_product = 1;
        for i in 0..length {
            products[i] = prefix_product;
            prefix_product *= nums[i];
        }

        let mut suffix_product = 1;
        for i in (0..length).rev() {
            products[i] *= suffix_product;
            suffix_product *= nums[i];
        }

        products
    }
}
