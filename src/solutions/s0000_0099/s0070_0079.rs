use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// 70. 爬楼梯
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut pre, mut prepre) = (1, 0);
        for _ in 0..n {
            let tmp = pre;
            pre += prepre;
            prepre = tmp;
        }
        pre
    }

    /// 71. 简化路径
    #[allow(dead_code)]
    pub fn simplify_path(path: String) -> String {
        let mut res = Vec::new();
        for item in path.split('/').filter(|e| !e.is_empty()) {
            if item.eq("..") {
                res.pop();
            } else if !item.eq(".") {
                res.push(item);
            }
        }
        format!("/{}", res.join("/"))
    }

    /// 72. 编辑距离
    #[allow(dead_code)]
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len());
        if n * m == 0 {
            return (m + n) as i32;
        }
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp.iter_mut().enumerate().for_each(|(i, v)| v[0] = i);
        for j in 1..n + 1 {
            dp[0][j] = j;
        }
        for (i, c1) in word1.chars().enumerate() {
            for (j, c2) in word2.chars().enumerate() {
                let down = dp[i + 1][j] + 1;
                let left = dp[i][j + 1] + 1;
                let mut left_down = dp[i][j];
                if !c1.eq(&c2) {
                    left_down += 1;
                }
                dp[i + 1][j + 1] = left.min(down).min(left_down);
            }
        }
        dp[m][n] as i32
    }

    /// 73. 矩阵置零
    #[allow(dead_code)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (nr, nc) = (matrix.len(), matrix[0].len());
        let first_row = (0..nc).any(|x| matrix[0][x] == 0);
        let first_col = (0..nr).any(|x| matrix[x][0] == 0);
        for i in 1..nr {
            for j in 1..nc {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }
        for j in 1..nc {
            if matrix[0][j] == 0 {
                matrix.iter_mut().take(nr).skip(1).for_each(|v| v[j] = 0);
            }
        }
        matrix.iter_mut().take(nr).skip(1).for_each(|v| {
            if v[0] == 0 {
                *v = vec![0; nc];
            }
        });

        if first_row {
            for j in 0..nc {
                matrix[0][j] = 0;
            }
        }
        if first_col {
            matrix.iter_mut().for_each(|v| v[0] = 0);
        }
    }

    /// 74. 搜索二维矩阵
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        let (mut l, mut r) = (0, m * n - 1);
        while l <= r {
            let p = (l + r) / 2;
            let (i, j) = (p / n, p % n);
            match matrix[i as usize][j as usize].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Greater => r = p - 1,
                Ordering::Less => l = p + 1,
            }
        }
        false
    }

    /// 75. 颜色分类
    #[allow(dead_code)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut i, mut one_flag, mut two_flag, mut overflow) = (0, 0, nums.len() - 1, false);
        while !overflow && i <= two_flag {
            match nums[i].cmp(&1) {
                Ordering::Equal => i += 1,
                Ordering::Less => {
                    nums.swap(one_flag, i);
                    one_flag += 1;
                    i += 1
                }
                Ordering::Greater => {
                    nums.swap(two_flag, i);
                    (two_flag, overflow) = two_flag.overflowing_sub(1);
                }
            }
        }
    }

    /// 76. 最小覆盖子串
    fn check(tchar_cnt: &HashMap<&u8, i32>, schar_cnt: &HashMap<u8, i32>) -> bool {
        for (k, v) in tchar_cnt.iter() {
            if let Some(c) = schar_cnt.get(k) {
                if c < v {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)]
    pub fn min_window(s: String, t: String) -> String {
        let (mut res, mut length) = (String::new(), s.len() + 1);
        let (mut l, mut r) = (0, 0);
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut tchar_cnt = HashMap::new();
        let mut schar_cnt = HashMap::new();
        t.iter().for_each(|b| {
            tchar_cnt.entry(b).and_modify(|cnt| *cnt += 1).or_insert(1);
        });
        while r < s.len() {
            schar_cnt
                .entry(s[r])
                .and_modify(|cnt| *cnt += 1)
                .or_insert(1);
            while Self::check(&tchar_cnt, &schar_cnt) {
                if r - l + 1 < length {
                    res = String::from_utf8(s[l..=r].to_vec()).unwrap();
                    length = r - l + 1;
                }
                schar_cnt.entry(s[l]).and_modify(|cnt| *cnt -= 1);
                l += 1;
            }
            r += 1;
        }
        res
    }

    /// 77. 组合
    #[allow(dead_code)]
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrace(l: i32, n: i32, k: i32, combine: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if combine.len() == k as usize {
                res.push(combine.clone());
            } else {
                for i in l..=n {
                    combine.push(i);
                    backtrace(i + 1, n, k, combine, res);
                    combine.pop();
                }
            }
        }
        let mut res = Vec::new();
        let mut combine = Vec::new();
        backtrace(1, n, k, &mut combine, &mut res);
        res
    }

    /// 78. 子集
    #[allow(dead_code)]
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrace(l: usize, nums: &Vec<i32>, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            for (i, num) in nums.iter().skip(l).enumerate() {
                subset.push(*num);
                res.push(subset.clone());
                backtrace(l + i + 1, nums, subset, res);
                subset.pop();
            }
        }
        let mut res = vec![vec![]];
        let mut subset = Vec::new();
        backtrace(0, &nums, &mut subset, &mut res);
        res
    }

    /// 79. 单词搜索
    #[allow(dead_code)]
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars = word.chars().collect::<Vec<char>>();
        let chars = chars.as_slice();
        fn backtrace(
            board: &Vec<Vec<char>>,
            left_word: &[char],
            pos: (i32, i32),
            visited: &mut Vec<Vec<bool>>,
            res: &mut bool,
        ) {
            if left_word.len() == 1 && left_word[0] == board[pos.0 as usize][pos.1 as usize] {
                *res = true;
            } else if left_word[0] == board[pos.0 as usize][pos.1 as usize] {
                [
                    (pos.0 + 1, pos.1),
                    (pos.0 - 1, pos.1),
                    (pos.0, pos.1 + 1),
                    (pos.0, pos.1 - 1),
                ]
                .iter()
                .for_each(|&(x, y)| {
                    if x >= 0
                        && x < board.len() as i32
                        && y >= 0
                        && y < board[0].len() as i32
                        && !visited[x as usize][y as usize]
                    {
                        visited[x as usize][y as usize] = true;
                        backtrace(board, &left_word[1..], (x, y), visited, res);
                        visited[x as usize][y as usize] = false;
                    }
                })
            }
        }
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let mut res = false;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                visited[i][j] = true;
                backtrace(&board, chars, (i as i32, j as i32), &mut visited, &mut res);
                visited[i][j] = false;
                if res {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 70. 爬楼梯
    #[test]
    fn climb_stairs() {
        let n = 2;
        let res = Solution::climb_stairs(n);
        assert_eq!(res, 2);
    }

    /// 71. 简化路径
    #[test]
    fn simplify_path() {
        let path = "/a/./b/../../c/".to_string();
        let res = Solution::simplify_path(path);
        assert_eq!(res, "/c".to_string());
    }

    /// 72. 编辑距离
    #[test]
    fn min_distance() {
        let word1 = "horse".to_string();
        let word2 = "ros".to_string();
        let res = Solution::min_distance(word1, word2);
        assert_eq!(res, 3);
    }

    /// 73. 矩阵置零
    #[test]
    fn set_zeroes() {
        let mut matrix = vec![vec![1], vec![0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0], vec![0]]);
    }

    /// 74. 搜索二维矩阵
    #[test]
    fn search_matrix() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        let res = Solution::search_matrix(matrix, target);
        assert!(res);
    }

    /// 75. 颜色分类
    #[test]
    fn sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    /// 76. 最小覆盖子串
    #[test]
    fn min_window() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let res = Solution::min_window(s, t);
        assert_eq!(res, "BANC".to_string());
    }

    /// 77. 组合
    #[test]
    fn combine() {
        let n = 4;
        let k = 2;
        let mut res = Solution::combine(n, k);
        let mut fx_res = vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ];
        res.sort_unstable();
        fx_res.sort_unstable();
        assert_eq!(res, fx_res)
    }

    /// 78. 子集
    #[test]
    fn subsets() {
        let nums = vec![1, 2, 3];
        let mut res = Solution::subsets(nums);
        let mut fx_res = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        res.sort_unstable();
        fx_res.sort_unstable();
        assert_eq!(res, fx_res);
    }

    /// 79. 单词搜索
    #[test]
    fn exist() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        let res = Solution::exist(board, word);
        assert!(res);
    }
}
