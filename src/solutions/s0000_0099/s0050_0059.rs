pub struct Solution {}

impl Solution {
    // 54. 螺旋矩阵

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

    // 55. 跳跃游戏
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // 54. 螺旋矩阵
    #[test]
    fn spiral_order() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let res = Solution::spiral_order(matrix);
        assert_eq!(res, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);

        let matrix = vec![vec![1]];
        let res = Solution::spiral_order(matrix);
        assert_eq!(res, vec![1]);
    }

    // 55. 跳跃游戏
    #[test]
    fn can_jump() {
        let nums = vec![2, 3, 1, 1, 4];
        let res = Solution::can_jump(nums);
        assert_eq!(res, true);

        let nums = vec![3, 2, 1, 0, 4];
        let res = Solution::can_jump(nums);
        assert_eq!(res, false);
    }
}
