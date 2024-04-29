use std::cmp::Ordering;

use crate::model::list::ListNode;

trait IsNumber {
    fn is_point(&self) -> bool;

    fn is_sign(&self) -> bool;

    fn is_exp(&self) -> bool;
}

impl IsNumber for char {
    fn is_point(&self) -> bool {
        matches!(*self, '.')
    }

    fn is_sign(&self) -> bool {
        matches!(*self, '+' | '-')
    }

    fn is_exp(&self) -> bool {
        matches!(*self, 'e' | 'E')
    }
}

enum State {
    Initial,
    IntSign,
    Integer,
    Point,
    PointWithoutInt,
    Fraction,
    Exp,
    ExpSign,
    ExpNumber,
    NotNumber,
}

impl State {
    fn is_accept(&self) -> bool {
        matches!(
            *self,
            Self::Integer | Self::Point | Self::Fraction | Self::ExpNumber
        )
    }

    fn trans(&mut self, c: char) {
        *self = match &self {
            Self::Initial => {
                if c.is_sign() {
                    Self::IntSign
                } else if c.is_ascii_digit() {
                    Self::Integer
                } else if c.is_point() {
                    Self::PointWithoutInt
                } else {
                    Self::NotNumber
                }
            }
            Self::IntSign => {
                if c.is_ascii_digit() {
                    Self::Integer
                } else if c.is_point() {
                    Self::PointWithoutInt
                } else {
                    Self::NotNumber
                }
            }
            Self::Integer => {
                if c.is_ascii_digit() {
                    Self::Integer
                } else if c.is_point() {
                    Self::Point
                } else if c.is_exp() {
                    Self::Exp
                } else {
                    Self::NotNumber
                }
            }
            Self::Point => {
                if c.is_ascii_digit() {
                    Self::Fraction
                } else if c.is_exp() {
                    Self::Exp
                } else {
                    Self::NotNumber
                }
            }
            Self::PointWithoutInt => {
                if c.is_ascii_digit() {
                    Self::Fraction
                } else {
                    Self::NotNumber
                }
            }
            Self::Fraction => {
                if c.is_ascii_digit() {
                    Self::Fraction
                } else if c.is_exp() {
                    Self::Exp
                } else {
                    Self::NotNumber
                }
            }
            Self::Exp => {
                if c.is_sign() {
                    Self::ExpSign
                } else if c.is_ascii_digit() {
                    Self::ExpNumber
                } else {
                    Self::NotNumber
                }
            }
            Self::ExpSign => {
                if c.is_ascii_digit() {
                    Self::ExpNumber
                } else {
                    Self::NotNumber
                }
            }
            Self::ExpNumber => {
                if c.is_ascii_digit() {
                    Self::ExpNumber
                } else {
                    Self::NotNumber
                }
            }
            Self::NotNumber => Self::NotNumber,
        }
    }
}

pub struct Solution {}

impl Solution {
    /// 60. 排列序列
    #[allow(dead_code)]
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut nums: Vec<String> = (1..=n).map(|x| x.to_string()).collect();
        let (mut cnts, mut multi) = (Vec::new(), 1);
        for i in 1..n {
            multi *= i;
            cnts.push(multi);
        }
        let mut res = String::new();
        while k > 1 {
            let cum = cnts.pop().unwrap();
            let index = ((k - 1) / cum) as usize;
            res.push_str(nums[index].as_str());
            nums.remove(index);
            k -= index as i32 * cum;
        }
        res.push_str(nums.concat().as_str());
        res
    }

    /// 61. 旋转链表
    #[allow(dead_code)]
    pub fn rotate_right(
        mut head: Option<Box<ListNode<i32>>>,
        k: i32,
    ) -> Option<Box<ListNode<i32>>> {
        if head.is_none() {
            return head;
        }
        let mut cur = head.as_ref();
        let mut n = 0;
        while let Some(node) = cur {
            n += 1;
            cur = node.next.as_ref();
        }
        let left = k % n;
        if left == 0 {
            return head;
        }
        let mut cur = head.as_mut().unwrap();
        for _ in 0..(n - left - 1) {
            cur = cur.next.as_mut().unwrap();
        }

        let mut new_head = cur.next.take();

        let mut cur = new_head.as_mut().unwrap();
        while cur.next.is_some() {
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = head;

        new_head
    }

    /// 62. 不同路径
    #[allow(dead_code)]
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut res = 1;
        let min = (m - 1).min(n - 1) as i64;
        let sum = (m + n - 2) as i64;
        for (a, b) in (sum - min + 1..=sum).zip(1..=min) {
            res = res * a / b;
        }
        res as i32
    }

    /// 63. 不同路径 II
    #[allow(dead_code)]
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (nr, nc) = (obstacle_grid.len(), obstacle_grid[0].len());
        for i in 0..nr {
            for j in 0..nc {
                if obstacle_grid[i][j] == 1 {
                    obstacle_grid[i][j] = 0;
                } else if i == 0 && j == 0 {
                    obstacle_grid[i][j] = 1;
                } else if i == 0 {
                    obstacle_grid[i][j] = obstacle_grid[i][j - 1];
                } else if j == 0 {
                    obstacle_grid[i][j] = obstacle_grid[i - 1][j];
                } else {
                    obstacle_grid[i][j] = obstacle_grid[i - 1][j] + obstacle_grid[i][j - 1];
                }
            }
        }
        obstacle_grid[nr - 1][nc - 1]
    }

    /// 64. 最小路径和
    #[allow(dead_code)]
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (nr, nc) = (grid.len(), grid[0].len());
        for i in 0..nr {
            for j in 0..nc {
                if i == 0 && j == 0 {
                    continue;
                } else if i == 0 {
                    grid[i][j] += grid[i][j - 1];
                } else if j == 0 {
                    grid[i][j] += grid[i - 1][j];
                } else {
                    grid[i][j] += grid[i - 1][j].min(grid[i][j - 1]);
                }
            }
        }
        grid[nr - 1][nc - 1]
    }

    /// 65. 有效数字
    #[allow(dead_code)]
    pub fn is_number(s: String) -> bool {
        let mut state = State::Initial;
        for c in s.chars() {
            if matches!(state, State::NotNumber) {
                return false;
            }
            state.trans(c)
        }
        state.is_accept()
    }

    /// 66. 加一
    #[allow(dead_code)]
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut remaind = 1;
        for i in (0..digits.len()).rev() {
            if remaind == 0 {
                return digits;
            }
            let sum = digits[i] + remaind;
            if sum == 10 {
                remaind = 1;
                digits[i] = 0;
            } else {
                remaind = 0;
                digits[i] = sum;
            }
        }
        if remaind == 1 {
            digits.insert(0, remaind)
        }
        digits
    }

    /// 67. 二进制求和
    #[allow(dead_code)]
    pub fn add_binary(a: String, b: String) -> String {
        let (mut a, mut b) = (a.as_bytes().to_vec(), b.as_bytes().to_vec());
        let mut res = String::new();
        let mut carry = 0;
        loop {
            match (a.pop(), b.pop()) {
                (Some(a), Some(b)) => {
                    let sum = a - b'0' + b - b'0' + carry;
                    carry = sum / 2;
                    let left = sum % 2;
                    res.insert(0, (b'0' + left) as char);
                }
                (None, Some(x)) | (Some(x), None) => {
                    if carry == 1 {
                        let sum = x - b'0' + carry;
                        carry = sum / 2;
                        let left = sum % 2;
                        res.insert(0, (b'0' + left) as char);
                    } else {
                        res.insert(0, x as char);
                    }
                }
                (None, None) => {
                    if carry == 1 {
                        res.insert(0, '1');
                    }
                    return res;
                }
            }
        }
    }

    /// 68. 文本左右对齐
    #[allow(dead_code)]
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut length = -1;
        let mut cur_words = Vec::new();
        let mut res = Vec::new();
        let usize_width = max_width as usize;
        for word in words {
            length += word.len() as i32 + 1;
            match length.cmp(&max_width) {
                Ordering::Equal => {
                    cur_words.push(word);
                    res.push(cur_words.join(" "));
                    cur_words = Vec::new();
                    length = -1;
                }
                Ordering::Less => {
                    cur_words.push(word);
                    continue;
                }
                Ordering::Greater => {
                    let extra_space = max_width - (length - word.len() as i32 - 1);
                    length = word.len() as i32;
                    let space_num = cur_words.len() - 1;
                    let mut cur_line = String::new();
                    if space_num == 0 {
                        res.push(format!("{:<usize_width$}", cur_words.first().unwrap()));
                    } else {
                        let (space, mut left) = (
                            extra_space as usize / space_num,
                            extra_space as usize % space_num,
                        );
                        for (i, w) in cur_words.iter().enumerate() {
                            if i == 0 {
                                cur_line.push_str(w);
                            } else {
                                let spaces = if left > 0 {
                                    left -= 1;
                                    vec![" "; space + 2].join("")
                                } else {
                                    vec![" "; space + 1].join("")
                                };
                                cur_line.push_str(&(spaces + w));
                            }
                        }
                        res.push(cur_line);
                    }
                    cur_words = vec![word];
                }
            }
        }
        if cur_words.is_empty() {
            if let Some(line) = res.pop() {
                res.push(
                    line.split(' ')
                        .filter(|x| !x.is_empty())
                        .collect::<Vec<&str>>()
                        .join(" "),
                );
            }
        } else {
            let cur_line = cur_words.join(" ");
            res.push(format!("{:<usize_width$}", cur_line));
        }
        res
    }

    /// 69. x的平方根
    ///
    /// # 二分查找
    #[allow(dead_code)]
    fn binary_search(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let (mut l, mut r, mut res) = (0, x, -1);
        while l <= r {
            let p = (l + r) / 2;
            if x / p >= p {
                res = p;
                l = p + 1;
            } else {
                r = p - 1;
            }
        }
        res
    }

    /// # 牛顿法
    #[allow(dead_code)]
    pub fn newton_method(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut x0 = x as f64 / 2.;
        loop {
            let x1 = 0.5 * (x as f64 / x0 + x0);
            if (x1 - x0).abs() < 1e-6 {
                break;
            }
            x0 = x1;
        }
        x0 as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::model::list::List;

    use super::*;

    /// 60. 排列序列
    #[test]
    fn get_permutation() {
        let n = 3;
        let k = 3;
        let res = Solution::get_permutation(n, k);
        assert_eq!(res, "213".to_string());
    }

    /// 61. 旋转链表
    #[test]
    fn rotate_right() {
        let vals = vec![1, 2, 3, 4, 5];
        let k = 2;
        let head = List::new(&vals);
        let res = Solution::rotate_right(head.head, k);
        let vectors = List { head: res }.as_vec();
        assert_eq!(vectors, vec![4, 5, 1, 2, 3]);
    }

    /// 62. 不同路径
    #[test]
    fn unique_paths() {
        let m = 3;
        let n = 7;
        let res = Solution::unique_paths(m, n);
        assert_eq!(res, 28);
    }

    /// 63. 不同路径 II
    #[test]
    fn unique_paths_with_obstacles() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let res = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(res, 2);
    }

    /// 64. 最小路径和
    #[test]
    fn min_path_sum() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let res = Solution::min_path_sum(grid);
        assert_eq!(res, 7);
    }

    /// 65. 有效数字
    #[test]
    fn is_number() {
        let number_strs = [
            "2",
            "0089",
            "-0.1",
            "+3.14",
            "4.",
            "-.9",
            "2e10",
            "-90E3",
            "3e+7",
            "+6e-1",
            "53.5e93",
            "-123.456e789",
        ];
        for str in number_strs {
            assert!(Solution::is_number(str.to_string()))
        }

        let not_number_strs = ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"];
        for str in not_number_strs {
            assert!(!Solution::is_number(str.to_string()))
        }
    }

    /// 66. 加一
    #[test]
    fn plus_one() {
        let digits = vec![1, 2, 3];
        let res = Solution::plus_one(digits);
        assert_eq!(res, vec![1, 2, 4]);
    }

    /// 67. 二进制求和
    #[test]
    fn add_binary() {
        let a = "11".to_string();
        let b = "1".to_string();
        let res = Solution::add_binary(a, b);
        assert_eq!(res, "100".to_string());
    }

    /// 68. 文本左右对齐
    #[test]
    fn full_justify() {
        let words = vec![
            "This".to_string(),
            "is".to_string(),
            "an".to_string(),
            "example".to_string(),
            "of".to_string(),
            "text".to_string(),
            "justification.".to_string(),
        ];
        let max_width = 16;
        let res = Solution::full_justify(words, max_width);
        assert_eq!(
            res,
            vec![
                "This    is    an".to_string(),
                "example  of text".to_string(),
                "justification.  ".to_string()
            ]
        )
    }

    /// 69. x的平方根
    #[test]
    pub fn my_sqrt() {
        let x = 4;
        let res = Solution::binary_search(x);
        assert_eq!(res, 2);

        let x = 8;
        let res = Solution::newton_method(x);
        assert_eq!(res, 2);
    }
}
