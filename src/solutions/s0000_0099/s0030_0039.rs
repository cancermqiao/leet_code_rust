pub struct Solution {}

impl Solution {
    /// 36. 有效的数独
    #[allow(dead_code)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![vec![0; 9]; 9];
        let mut cols = vec![vec![0; 9]; 9];
        let mut sub_boxes = vec![vec![0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let index = board[i][j] as usize - '0' as usize - 1;
                    rows[i][index] += 1;
                    cols[j][index] += 1;
                    sub_boxes[i / 3 * 3 + j / 3][index] += 1;

                    if rows[i][index] > 1
                        || cols[j][index] > 1
                        || sub_boxes[i / 3 * 3 + j / 3][index] > 1
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 36. 有效的数独
    #[test]
    fn is_valid_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let res = Solution::is_valid_sudoku(board);
        assert_eq!(res, true);
    }
}
