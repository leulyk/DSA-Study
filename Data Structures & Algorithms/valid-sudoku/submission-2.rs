impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut sub_boxes = [0u16; 9];

        for row in 0..9 {
            for col in 0..9 {
                let current = board[row][col];

                if current == '.' {
                    continue;
                }
                
                let box_index = (row / 3) + 3 * (col / 3);
                let num = current.to_string().parse::<u16>().unwrap();
                let offset = 1u16 << num - 1;

                if rows[row] & offset > 0 || cols[col] & offset > 0 || 
                    sub_boxes[box_index] & offset > 0 {
                    return false;
                }

                rows[row] |= offset;
                cols[col] |= offset;
                sub_boxes[box_index] |= offset;
            }
        }

        true
    }
}
