pub struct Solution {}

impl Solution {
    /// 1329. 将矩阵按对角线排序
    fn mat_swap(mat: &mut [Vec<i32>], a: i32, b: i32, diagonal_difference: i32) {
        let tmp = mat[(a + diagonal_difference) as usize][a as usize];
        mat[(a + diagonal_difference) as usize][a as usize] =
            mat[(b + diagonal_difference) as usize][b as usize];
        mat[(b + diagonal_difference) as usize][b as usize] = tmp;
    }

    fn _diagonal_sort(mat: &mut Vec<Vec<i32>>, left: i32, right: i32, diagonal_difference: i32) {
        if left >= right {
            return;
        }
        let (pivot, mut l, mut r) = (left, left, right);
        while l < r {
            while l < r
                && mat[(r + diagonal_difference) as usize][r as usize]
                    >= mat[(pivot + diagonal_difference) as usize][pivot as usize]
            {
                r -= 1;
            }
            while l < r
                && mat[(l + diagonal_difference) as usize][l as usize]
                    <= mat[(pivot + diagonal_difference) as usize][pivot as usize]
            {
                l += 1;
            }
            Self::mat_swap(mat, l, r, diagonal_difference);
        }
        Self::mat_swap(mat, l, pivot, diagonal_difference);
        if left + 1 < l {
            Self::_diagonal_sort(mat, left, l - 1, diagonal_difference);
        }
        if l + 1 < right {
            Self::_diagonal_sort(mat, l + 1, right, diagonal_difference);
        }
    }

    #[allow(dead_code)]
    pub fn diagonal_sort(mat: &mut Vec<Vec<i32>>) {
        let (m, n) = (mat.len() as i32, mat[0].len() as i32);
        for diagnal_difference in (-n + 1)..m {
            match diagnal_difference.cmp(&0) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Self::_diagonal_sort(
                    mat,
                    0 - diagnal_difference,
                    (m - diagnal_difference).min(n) - 1,
                    diagnal_difference,
                ),
                std::cmp::Ordering::Greater => Self::_diagonal_sort(
                    mat,
                    0,
                    (m - diagnal_difference).min(n) - 1,
                    diagnal_difference,
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1329. 将矩阵按对角线排序
    #[test]
    fn diagonal_sort() {
        let mut mat = vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]];
        Solution::diagonal_sort(&mut mat);
        assert_eq!(
            mat,
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]
        );
    }
}
