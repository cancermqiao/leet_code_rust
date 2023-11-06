pub struct Solution {}

impl Solution {
    /// 50. Pow(x, n)
    fn quick_mul(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.;
        }
        let res = Self::quick_mul(x, n / 2);
        if n % 2 == 1 {
            res * res * x
        } else {
            res * res
        }
    }

    #[allow(dead_code)]
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n >= 0 {
            Self::quick_mul(x, n as i64)
        } else {
            1. / Self::quick_mul(x, -(n as i64))
        }
    }

    /// 51. N皇后
    fn build_board(buf: &Vec<usize>) -> Vec<String> {
        let mut res = Vec::new();
        for &i in buf {
            let mut init = vec!["."; buf.len()];
            init[i] = "Q";
            res.push(init.join(""));
        }
        res
    }

    fn backtrace(
        cols: &mut Vec<bool>,
        diagonal1: &mut Vec<bool>,
        diagonal2: &mut Vec<bool>,
        buf: &mut Vec<usize>,
        res: &mut Vec<Vec<String>>,
        row: usize,
    ) {
        let n = cols.len();
        if row == n {
            res.push(Self::build_board(buf));
        } else {
            for i in 0..n {
                if cols[i] && diagonal1[row + i] && diagonal2[row + n - i - 1] {
                    cols[i] = false;
                    diagonal1[row + i] = false;
                    diagonal2[row + n - i - 1] = false;
                    buf.push(i);
                    Self::backtrace(cols, diagonal1, diagonal2, buf, res, row + 1);
                    buf.pop();
                    cols[i] = true;
                    diagonal1[row + i] = true;
                    diagonal2[row + n - i - 1] = true;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let (mut buf, mut res) = (Vec::new(), Vec::new());
        let (mut cols, mut diagonal1, mut diagonal2) =
            (vec![true; n], vec![true; 2 * n - 1], vec![true; 2 * n - 1]);
        Self::backtrace(
            &mut cols,
            &mut diagonal1,
            &mut diagonal2,
            &mut buf,
            &mut res,
            0,
        );
        res
    }

    /// 52. N皇后 II
    fn backtrace_total(
        cols: &mut Vec<bool>,
        diagonal1: &mut Vec<bool>,
        diagonal2: &mut Vec<bool>,
        buf: &mut Vec<usize>,
        res: &mut i32,
        row: usize,
    ) {
        let n = cols.len();
        if row == n {
            *res += 1;
        } else {
            for i in 0..n {
                if cols[i] && diagonal1[row + i] && diagonal2[row + n - i - 1] {
                    cols[i] = false;
                    diagonal1[row + i] = false;
                    diagonal2[row + n - i - 1] = false;
                    buf.push(i);
                    Self::backtrace_total(cols, diagonal1, diagonal2, buf, res, row + 1);
                    buf.pop();
                    cols[i] = true;
                    diagonal1[row + i] = true;
                    diagonal2[row + n - i - 1] = true;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let (mut buf, mut res) = (Vec::new(), 0);
        let (mut cols, mut diagonal1, mut diagonal2) =
            (vec![true; n], vec![true; 2 * n - 1], vec![true; 2 * n - 1]);
        Self::backtrace_total(
            &mut cols,
            &mut diagonal1,
            &mut diagonal2,
            &mut buf,
            &mut res,
            0,
        );
        res
    }

    /// 53. 最大子数组和
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut pre, mut res) = (0, i32::MIN);
        for num in nums {
            let cur = num + pre.max(0);
            res = res.max(cur);
            pre = cur.max(0);
        }
        res
    }

    /// 54. 螺旋矩阵
    #[allow(dead_code)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut top, mut bottom, mut left, mut right) =
            (0, matrix.len() as i32 - 1, 0, matrix[0].len() as i32 - 1);
        let mut res = Vec::new();
        while top <= bottom && left <= right {
            // RIGHT
            for i in left..=right {
                res.push(matrix[top as usize][i as usize])
            }
            // DOWN
            for i in top + 1..=bottom {
                res.push(matrix[i as usize][right as usize])
            }
            // LEFT
            if bottom > top {
                for i in (left..right).rev() {
                    res.push(matrix[bottom as usize][i as usize])
                }
            }
            // UP
            if right > left {
                for i in (top + 1..bottom).rev() {
                    res.push(matrix[i as usize][left as usize])
                }
            }
            // update index
            top += 1;
            bottom -= 1;
            left += 1;
            right -= 1;
        }
        res
    }

    /// 55. 跳跃游戏
    #[allow(dead_code)]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut step = 1;
        for i in (0..(nums.len() - 1)).rev() {
            if nums[i] < step {
                step += 1;
            } else {
                step = 1;
            }
        }
        step == 1
    }

    /// 56. 合并区间
    #[allow(dead_code)]
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|k| k[0]);

        let mut merged_intervals: Vec<Vec<i32>> = Vec::new();
        for interval in intervals {
            if let Some(last_interval) = merged_intervals.last_mut() {
                if last_interval[1] >= interval[0] {
                    last_interval[1] = last_interval[1].max(interval[1]);
                } else {
                    merged_intervals.push(interval);
                }
            } else {
                merged_intervals.push(interval);
            }
        }
        merged_intervals
    }

    /// 57. 插入区间
    #[allow(dead_code)]
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut left, mut right) = (new_interval[0], new_interval[1]);
        let mut res = Vec::new();
        let mut inserted = false;
        for interval in intervals {
            if interval[0] > right {
                // 插入区间的右侧无交集
                if !inserted {
                    res.push(vec![left, right]);
                    inserted = true;
                }
                res.push(interval);
            } else if interval[1] < left {
                // 插入区间的左侧无交集
                res.push(interval);
            } else {
                left = left.min(interval[0]);
                right = right.max(interval[1]);
            }
        }
        if !inserted {
            res.push(vec![left, right]);
        }
        res
    }

    /// 58. 最后一个单次
    #[allow(dead_code)]
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res = 0;
        for c in s.chars().rev() {
            if c != ' ' {
                res += 1;
            } else if res > 0 {
                return res;
            }
        }
        res
    }

    /// 59. 螺旋矩阵 II
    #[allow(dead_code)]
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 1 {
            return vec![vec![1]];
        }
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];
        let (mut left, mut right, mut top, mut bottom, mut num) = (0, n - 1, 0, n - 1, 1);
        while left <= right && top <= bottom {
            // RIGHT
            for i in left..=right {
                res[top][i] = num;
                num += 1;
            }

            // DOWN
            for i in top + 1..=bottom {
                res[i][right] = num;
                num += 1;
            }

            // LEFT
            if top < bottom {
                for i in (left..right).rev() {
                    res[bottom][i] = num;
                    num += 1;
                }
            }

            // UP
            if left < right {
                for i in (top + 1..bottom).rev() {
                    res[i][left] = num;
                    num += 1;
                }
            }
            top += 1;
            bottom -= 1;
            left += 1;
            right -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 50. Pow(x, n)
    #[test]
    pub fn my_pow() {
        let x = 2.00000;
        let n = 10;
        let res = Solution::my_pow(x, n);
        assert_eq!(res, 1024.0);
    }

    /// 51. N皇后
    #[test]
    fn solve_n_queens() {
        let n = 4;
        let res = Solution::solve_n_queens(n);
        assert_eq!(
            res,
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
    }

    /// 52. N皇后 II
    #[test]
    fn total_n_queens() {
        let n = 4;
        let res = Solution::total_n_queens(n);
        assert_eq!(res, 2);
    }

    /// 53. 最大子数组和
    #[test]
    fn max_sub_array() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let res = Solution::max_sub_array(nums);
        assert_eq!(res, 6);
    }

    /// 54. 螺旋矩阵
    #[test]
    fn spiral_order() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let res = Solution::spiral_order(matrix);
        assert_eq!(res, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);

        let matrix = vec![vec![1]];
        let res = Solution::spiral_order(matrix);
        assert_eq!(res, vec![1]);
    }

    /// 55. 跳跃游戏
    #[test]
    fn can_jump() {
        let nums = vec![2, 3, 1, 1, 4];
        let res = Solution::can_jump(nums);
        assert!(res);

        let nums = vec![3, 2, 1, 0, 4];
        let res = Solution::can_jump(nums);
        assert!(!res);
    }

    /// 56. 合并区间
    #[test]
    fn merge() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let merged_intervals = Solution::merge(intervals);
        assert_eq!(
            merged_intervals,
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    /// 57. 插入区间
    #[test]
    fn insert() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let res = Solution::insert(intervals, new_interval);
        assert_eq!(res, vec![vec![1, 5], vec![6, 9]]);
    }

    /// 58. 最后一个单次
    #[test]
    fn length_of_last_word() {
        let s = "Hello World".to_string();
        let res = Solution::length_of_last_word(s);
        assert_eq!(res, 5);
    }

    /// 59. 螺旋矩阵 II
    #[test]
    fn generate_matrix() {
        let n = 3;
        let res = Solution::generate_matrix(n);
        assert_eq!(res, vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]);
    }
}
