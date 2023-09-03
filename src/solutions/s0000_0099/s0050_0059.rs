pub struct Solution {}

impl Solution {
    /// 54. 螺旋矩阵
    #[allow(dead_code)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut top, mut bottom, mut left, mut right) =
            (0, matrix.len() as i32 - 1, 0, matrix[0].len() as i32 - 1);
        let mut res = Vec::new();
        while top <= bottom && left <= right {
            // LEFT
            for i in left..=right {
                res.push(matrix[top as usize][i as usize])
            }
            // DOWN
            for i in top + 1..=bottom {
                res.push(matrix[i as usize][right as usize])
            }
            // RIGHT
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
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
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
                return res
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(res, true);

        let nums = vec![3, 2, 1, 0, 4];
        let res = Solution::can_jump(nums);
        assert_eq!(res, false);
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
        let intervals = vec![vec![1,3],vec![6,9]];
        let new_interval = vec![2,5];
        let res = Solution::insert(intervals, new_interval);
        assert_eq!(res, vec![vec![1,5],vec![6,9]]);
    }

    /// 58. 最后一个单次
    #[test]
    fn length_of_last_word() {
        let s = "Hello World".to_string();
        let res = Solution::length_of_last_word(s);
        assert_eq!(res, 5);
    }
}
