use std::cmp::Ordering::{Equal, Greater, Less};

pub struct Solution;

impl Solution {
    /// 30. 串联所有单词的子串
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res = Vec::new();
        let (m, n, ls) = (words[0].len(), words.len(), s.len());
        for i in 0..m {
            if i + m * n > ls {
                break;
            }
            let mut count = std::collections::HashMap::new();
            for j in 0..n {
                let word = &s[i + j * m..i + (j + 1) * m];
                count.insert(word, count.get(word).unwrap_or(&0) + 1);
            }
            for word in &words {
                match count.get(word.as_str()) {
                    None => count.insert(word, -1),
                    Some(1) => count.remove(word.as_str()),
                    Some(a) => count.insert(word, a - 1),
                };
            }
            for start in (i..(ls - m * n + 1)).step_by(m) {
                if start != i {
                    let word = &s[start + (n - 1) * m..start + n * m];
                    match count.get(word) {
                        None => count.insert(word, 1),
                        Some(-1) => count.remove(word),
                        Some(a) => count.insert(word, a + 1),
                    };
                    let word = &s[start - m..start];
                    match count.get(word) {
                        None => count.insert(word, -1),
                        Some(1) => count.remove(word),
                        Some(a) => count.insert(word, a - 1),
                    };
                }
                if count.is_empty() {
                    res.push(start as i32);
                }
            }
        }
        res
    }

    /// 31. 下一个排列
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() as i32 - 2;
        while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
            i -= 1;
        }
        if i >= 0 {
            let mut j = nums.len() - 1;
            while nums[i as usize] >= nums[j] {
                j -= 1;
            }
            nums.swap(i as usize, j);
        }
        let (mut l, mut r) = (i + 1, nums.len() as i32 - 1);
        while l < r {
            nums.swap(l as usize, r as usize);
            l += 1;
            r -= 1;
        }
    }

    /// 32. 最长有效括号
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res = 0;
        let mut stack = vec![0];
        for (i, &v) in s.iter().enumerate() {
            if v == b'(' {
                stack.push(i + 1);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i + 1);
                } else {
                    res = res.max(i + 1 - stack.last().unwrap());
                }
            }
        }
        res as i32
    }

    /// 33. 搜索旋转排序数组
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let p = (l + r) / 2;
            if nums[p] == target {
                return p as i32;
            }
            if nums[p] < nums[l] {
                if target > nums[p] && nums[r] >= target {
                    l = p + 1;
                } else {
                    r = p - 1;
                }
            } else if target < nums[p] && nums[l] <= target {
                r = p - 1;
            } else {
                l = p + 1;
            }
        }
        -1
    }

    /// 34. 在排序数组中查找元素的第一个和最后一个位置
    ///
    /// 二分查找第一个大于或者大于等于target的下标
    fn binary_search(nums: &[i32], target: i32, equal: bool) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32 - 1);
        while l <= r {
            let p = (l + r) / 2;
            match nums[p as usize].cmp(&target) {
                Equal => {
                    if equal {
                        r = p - 1;
                    } else {
                        l = p + 1;
                    }
                }
                Less => l = p + 1,
                Greater => r = p - 1,
            }
        }
        l
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left_index = Solution::binary_search(&nums, target, true);
        let right_index = Solution::binary_search(&nums, target, false) - 1;
        if left_index <= right_index {
            vec![left_index, right_index]
        } else {
            vec![-1, -1]
        }
    }

    /// 35. 搜索插入位置
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0_i32, nums.len() as i32 - 1);
        while l <= r {
            let p = (l + r) / 2;
            match nums[p as usize].cmp(&target) {
                std::cmp::Ordering::Equal => return p,
                std::cmp::Ordering::Less => l = p + 1,
                std::cmp::Ordering::Greater => r = p - 1,
            }
        }
        l
    }

    /// 36. 有效的数独
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

    /// 37. 解数独
    fn dfs(
        board: &mut Vec<Vec<char>>,
        spaces: &Vec<(usize, usize)>,
        rows: &mut Vec<Vec<bool>>,
        cols: &mut Vec<Vec<bool>>,
        sub_boxes: &mut Vec<Vec<bool>>,
        pos: usize,
        valid: &mut bool,
    ) {
        if pos == spaces.len() {
            *valid = true;
            return;
        }
        let (i, j) = spaces[pos];
        for digit in 0..9 {
            if !rows[i][digit] && !cols[j][digit] && !sub_boxes[i / 3 * 3 + j / 3][digit] {
                rows[i][digit] = true;
                cols[j][digit] = true;
                sub_boxes[i / 3 * 3 + j / 3][digit] = true;
                board[i][j] = (b'0' + digit as u8 + 1) as char;
                Self::dfs(board, spaces, rows, cols, sub_boxes, pos + 1, valid);
                rows[i][digit] = false;
                cols[j][digit] = false;
                sub_boxes[i / 3 * 3 + j / 3][digit] = false;
            }
            if *valid {
                return;
            }
        }
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = vec![vec![false; 9]; 9];
        let mut cols = vec![vec![false; 9]; 9];
        let mut sub_boxes = vec![vec![false; 9]; 9];
        let mut spaces = Vec::new();
        let mut valid = false;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == '.' {
                    spaces.push((i, j));
                } else {
                    let digit = board[i][j].to_digit(10).unwrap() as usize - 1;
                    rows[i][digit] = true;
                    cols[j][digit] = true;
                    sub_boxes[i / 3 * 3 + j / 3][digit] = true;
                }
            }
        }
        Self::dfs(
            board,
            &spaces,
            &mut rows,
            &mut cols,
            &mut sub_boxes,
            0,
            &mut valid,
        );
    }

    /// 38. 外观数列
    ///
    /// # 递归
    fn recursion(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let s = Self::recursion(n - 1);
        let (mut pre, mut cnt, mut res) = (' ', 0, "".to_string());
        for c in s.chars() {
            if cnt == 0 {
                pre = c;
                cnt += 1;
            } else if c == pre {
                cnt += 1;
            } else {
                res.push_str(&format!("{}{}", cnt, pre));
                cnt = 1;
                pre = c;
            }
        }
        res.push_str(&format!("{}{}", cnt, pre));
        res
    }

    /// # 迭代
    fn iteration(n: i32) -> String {
        let mut res = "1".to_string();
        for _ in 1..n {
            let (mut pre, mut cnt, mut tmp) = (' ', 0, "".to_string());
            for c in res.chars() {
                if cnt == 0 {
                    pre = c;
                    cnt += 1;
                } else if c == pre {
                    cnt += 1;
                } else {
                    tmp.push_str(&format!("{}{}", cnt, pre));
                    cnt = 1;
                    pre = c;
                }
            }
            tmp.push_str(&format!("{}{}", cnt, pre));
            res = tmp;
        }
        res
    }

    pub fn count_and_say(n: i32, method_type: &str) -> String {
        match method_type {
            "recursion" => Self::recursion(n),
            "iteration" => Self::iteration(n),
            _ => Self::iteration(n),
        }
    }

    /// 39. 组合总和
    fn backtrace(
        candidates: &Vec<i32>,
        target: i32,
        buf: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        idx: usize,
    ) {
        if idx == candidates.len() {
            return;
        }
        if target == 0 {
            res.push(buf.clone());
            return;
        }
        Self::backtrace(candidates, target, buf, res, idx + 1);
        if target - candidates[idx] >= 0 {
            buf.push(candidates[idx]);
            Self::backtrace(candidates, target - candidates[idx], buf, res, idx);
            buf.pop();
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let (mut buf, mut res) = (Vec::new(), Vec::new());
        Self::backtrace(&candidates, target, &mut buf, &mut res, 0);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 30. 串联所有单词的子串
    #[test]
    fn find_substring() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let res = Solution::find_substring(s, words);
        assert_eq!(res, vec![0, 9]);
    }

    /// 31. 下一个排列
    #[test]
    fn next_permutation() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    /// 32. 最长有效括号
    #[test]
    fn longest_valid_parentheses() {
        let s = "(()".to_string();
        let res = Solution::longest_valid_parentheses(s);
        assert_eq!(res, 2);
    }

    /// 33. 搜索旋转排序数组
    #[test]
    fn search() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let res = Solution::search(nums, target);
        assert_eq!(res, 4);
    }

    /// 34. 在排序数组中查找元素的第一个和最后一个位置
    #[test]
    fn search_range() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let res = Solution::search_range(nums, target);
        assert_eq!(res, vec![3, 4]);
    }

    /// 35. 搜索插入位置
    #[test]
    fn search_insert() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let res = Solution::search_insert(nums, target);
        assert_eq!(res, 2);
    }

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
        assert!(res);
    }

    /// 37. 解数独
    #[test]
    fn solve_sudoku() {
        let mut board = vec![
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
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );
    }

    /// 38. 外观数列
    #[test]
    fn count_and_say() {
        let n = 4;
        let res = Solution::count_and_say(n, "recursion");
        assert_eq!(res, "1211");
        let res = Solution::count_and_say(n, "iteration");
        assert_eq!(res, "1211");
    }

    /// 39. 组合总和
    #[test]
    fn combination_sum() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let mut res = Solution::combination_sum(candidates, target);
        res.sort();
        assert_eq!(res, vec![vec![2, 2, 3], vec![7]]);
    }
}
