pub struct Solution {}

impl Solution {
    /// 283. 移动零
    #[allow(dead_code)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_flag = 0_usize;
        for index in 0..nums.len() {
            if nums[index] != 0 {
                nums[zero_flag] = nums[index];
                nums[index] = 0;
                zero_flag += 1
            }
        }
    }

    /// 289. 生命游戏
    ///
    /// 增加状态：
    /// 1. -1：原来死 -> 现在活
    /// 2. 2：原来活 -> 现在死
    #[allow(dead_code)]
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let (nr, nc) = (board.len() as i32, board[0].len() as i32);
        for r in 0..nr {
            for c in 0..nc {
                let mut live_num = 0;
                for i in -1..2 {
                    for j in -1..2 {
                        if (i == 0 && j == 0)
                            || (r + i < 0 || r + i >= nr)
                            || (c + j < 0 || c + j >= nc)
                        {
                            continue;
                        }
                        if board[(r + i) as usize][(c + j) as usize] > 0 {
                            live_num += 1;
                        }
                    }
                }
                if board[r as usize][c as usize] > 0 && live_num != 2 && live_num != 3 {
                    board[r as usize][c as usize] = 2;
                }
                if board[r as usize][c as usize] <= 0 && live_num == 3 {
                    board[r as usize][c as usize] = -1;
                }
            }
        }
        board.iter_mut().take(nr as usize).for_each(|v| {
            v.iter_mut().take(nc as usize).for_each(|x| {
                if *x == 2 {
                    *x = 0;
                }
                if *x == -1 {
                    *x = 1;
                }
            })
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 283. 移动零
    #[test]
    fn move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    /// 289. 生命游戏
    #[test]
    fn game_of_life() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(
            board,
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
        );
    }
}
