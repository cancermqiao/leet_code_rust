pub struct Solution {}

impl Solution {
    /// 60. 第k个排列
    pub fn get_permutation(n: i32, k: i32) -> String {
        unimplemented!()
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
    use super::*;

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

    /// 66. 加一
    #[test]
    fn plus_one() {
        let digits = vec![1, 2, 3];
        let res = Solution::plus_one(digits);
        assert_eq!(res, vec![1, 2, 4]);
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
