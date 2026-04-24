use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::new();
        let row_len = board.len();
        let col_len = board[0].len();

        // check each row
        for col in 0..col_len {
            set.clear(); 
            for row in 0..row_len {
                if Self::in_set(&board, row, col, &mut set) {
                    return false;
                }
            }
        }        

        // check each column
        for row in 0..row_len {
            set.clear(); 
            for col in 0..col_len {
                if Self::in_set(&board, row, col, &mut set) {
                    return false;
                }
            }
        }        

        // check sub-boxes
        for row in (1..row_len).step_by(3) {
            for col in (1..col_len).step_by(3) {
                set.clear(); 
                if board[row][col] != '.' {
                    set.insert(board[row][col]);
                }

                if Self::in_set(&board, row - 1, col - 1, &mut set) {
                    return false;
                }

                if Self::in_set(&board, row - 1, col, &mut set) {
                    return false;
                }
                
                if Self::in_set(&board, row - 1, col + 1, &mut set) {
                    return false;
                }
                
                if Self::in_set(&board, row, col - 1, &mut set) {
                    return false;
                }
                
                if Self::in_set(&board, row, col + 1, &mut set) {
                    return false;
                }
                
                if Self::in_set(&board, row + 1, col - 1, &mut set) {
                    return false;
                }
                
                if Self::in_set(&board, row + 1, col, &mut set) {
                    return false;
                }
                
                if Self::in_set(&board, row + 1, col + 1, &mut set) {
                    return false;
                }
                
            }
        }

        true
    }

    fn in_set(board: &Vec<Vec<char>>, row: usize, col: usize, set: &mut HashSet<char>) -> bool {
        if board[row][col] != '.' {
            match set.get(&board[row][col]) {
                Some(_) => return true,
                None => {
                    set.insert(board[row][col]);
                    return false
                }
            }
        }

        false
    }
}
