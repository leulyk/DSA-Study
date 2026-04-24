use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let len = board.len();
        let mut row_sets = vec![HashSet::new(); len];
        let mut col_sets = vec![HashSet::new(); len];
        let mut sub_box_sets = vec![HashSet::new(); len];

        for row in 0..len {
            for col in 0..len {
                let current = &board[row][col];

                if current == &'.' {
                    continue;
                }

                let sub_box_index = (row / 3) * 3 + (col / 3);

                if row_sets[row].contains(&current) || col_sets[col].contains(&current) ||
                    sub_box_sets[sub_box_index].contains(&current) {
                    return false;
                }

                row_sets[row].insert(current);
                col_sets[col].insert(current);
                sub_box_sets[sub_box_index].insert(current);
            }
        }

        true
    }
}
