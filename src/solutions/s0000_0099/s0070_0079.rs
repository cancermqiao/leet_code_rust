pub struct Solution {}

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

    /// 73. 矩阵置零
    #[test]
    fn set_zeroes() {
        let mut matrix = vec![vec![1], vec![0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0], vec![0]]);
    }
}
