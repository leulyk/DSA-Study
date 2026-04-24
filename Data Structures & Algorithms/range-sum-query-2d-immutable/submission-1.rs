struct NumMatrix {
    matrix: Vec<Vec<i32>>,
    prefix_sum: Vec<Vec<i32>>
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (row_len, col_len) = (matrix.len(), matrix[0].len());
        let mut prefix_sum = vec![vec![0; col_len]; row_len];

        for i in 0..row_len {
            for j in 0..col_len {
                let mut sum = matrix[i][j];

                if i >= 1 && j >= 1 {
                    sum -= prefix_sum[i - 1][j - 1];
                } 
                if i >= 1 {
                    sum += prefix_sum[i - 1][j];
                }
                if j >= 1 {
                    sum += prefix_sum[i][j - 1];
                }
                prefix_sum[i][j] = sum;
            }
        }

        Self {
            matrix,
            prefix_sum
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;

        let mut region_sum = self.prefix_sum[row2][col2];

        if row1 >= 1 {
            region_sum -= self.prefix_sum[row1 - 1][col2];
        }
        if col1 >= 1 {
            region_sum -= self.prefix_sum[row2][col1 - 1];
        }
        if row1 >= 1 && col1>= 1 {
            region_sum += self.prefix_sum[row1 - 1][col1 - 1];
        }

        region_sum
    }
}
