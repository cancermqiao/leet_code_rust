pub struct Solution {}

impl Solution {
    /// 130. 被围绕的区域
    #[allow(dead_code)]
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (nr, nc) = (board.len(), board[0].len());
        let mut stack = Vec::new();
        for (i, v) in board.iter_mut().enumerate() {
            if v[0] == 'O' {
                stack.push((i, 0));
                v[0] = 'A';
            }
            if v[nc - 1] == 'O' {
                stack.push((i, nc - 1));
                v[nc - 1] = 'A';
            }
        }
        for j in 1..nc {
            if board[0][j] == 'O' {
                stack.push((0, j));
                board[0][j] = 'A';
            }
            if board[nr - 1][j] == 'O' {
                stack.push((nr - 1, j));
                board[nr - 1][j] = 'A';
            }
        }
        while let Some((x, y)) = stack.pop() {
            for (nx, ny) in [
                (x as i32 + 1, y as i32),
                (x as i32 - 1, y as i32),
                (x as i32, y as i32 + 1),
                (x as i32, y as i32 - 1),
            ] {
                if nx >= 0
                    && nx < nr as i32
                    && ny >= 0
                    && ny < nc as i32
                    && board[nx as usize][ny as usize] == 'O'
                {
                    stack.push((nx as usize, ny as usize));
                    board[nx as usize][ny as usize] = 'A';
                }
            }
        }
        board.iter_mut().for_each(|v| {
            v.iter_mut().for_each(|x| {
                if *x == 'A' {
                    *x = 'O'
                } else if *x == 'O' {
                    *x = 'X'
                }
            })
        });
    }

    /// 134. 加油站
    #[allow(dead_code)]
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut i, n) = (0, gas.len());
        while i < n {
            let (mut cnt, mut sum_of_gas, mut sum_of_cost) = (0, 0, 0);
            while cnt < n {
                let index = (i + cnt) % n;
                sum_of_gas += gas[index];
                sum_of_cost += cost[index];
                if sum_of_gas < sum_of_cost {
                    break;
                }
                cnt += 1;
            }

            if cnt == n {
                return i as i32;
            } else {
                i = i + cnt + 1;
            }
        }
        -1
    }

    /// 135. 分发糖果
    #[allow(dead_code)]
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy_num = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candy_num[i] = candy_num[i - 1] + 1;
            }
        }
        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candy_num[i] = candy_num[i].max(candy_num[i + 1] + 1);
            }
        }
        candy_num.iter().sum()
    }

    /// 136. 只出现一次的数字
    #[allow(dead_code)]
    pub fn single_number1(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, num| acc ^ num)
    }

    /// 137. 只出现一次的数字 II
    #[allow(dead_code)]
    pub fn single_number2(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let mut count = 0;
            for &num in nums.iter() {
                count += (num >> i) & 1;
            }
            if count % 3 != 0 {
                res |= 1 << i;
            }
        }
        res
    }

    /// 139. 单次拆分
    #[allow(dead_code)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = word_dict.iter().map(|x| x.len()).max().unwrap();
        let mut dp = vec![0];
        for i in 0..s.len() {
            if dp.is_empty() {
                return false;
            }
            for &j in dp.iter() {
                if word_dict.contains(&s[j..i + 1].to_string()) {
                    dp.insert(0, i + 1);
                    break;
                }
            }
            if dp.last().unwrap() + n - 1 <= i {
                dp.pop();
            }
        }
        if let Some(&n) = dp.first() {
            n == s.len()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 130. 被围绕的区域
    #[test]
    fn solve() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X']
            ]
        );
    }

    /// 134. 加油站
    #[test]
    fn can_complete_circuit() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        let res = Solution::can_complete_circuit(gas, cost);
        assert_eq!(res, 3);
    }

    /// 135. 分发糖果
    #[test]
    fn candy() {
        let ratings = vec![1, 0, 2];
        let res = Solution::candy(ratings);
        assert_eq!(res, 5);
    }

    /// 136. 只出现一次的数字
    #[test]
    fn single_number1() {
        let nums = vec![2, 2, 1];
        let res = Solution::single_number1(nums);
        assert_eq!(res, 1);
    }

    /// 137. 只出现一次的数字 II
    #[test]
    fn single_number2() {
        let nums = vec![2, 2, 3, 2];
        let res = Solution::single_number2(nums);
        assert_eq!(res, 3);
    }

    /// 139. 单次拆分
    #[test]
    fn word_break() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        let res = Solution::word_break(s, word_dict);
        assert!(res);
    }
}
