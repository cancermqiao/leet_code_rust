use std::cmp::Ordering::{Equal, Greater, Less};

pub struct Solution {}

impl Solution {
    /// 31. 下一个排列
    #[allow(dead_code)]
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() as i32 - 2;
        while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
            i -= 1;
        }
        if i >= 0 {
            let mut j = nums.len() - 1;
            while nums[i as usize] >= nums[j as usize] {
                j -= 1;
            }
            nums.swap(i as usize, j as usize);
        }
        let (mut l, mut r) = (i + 1, nums.len() as i32 - 1);
        while l < r {
            nums.swap(l as usize, r as usize);
            l += 1;
            r -= 1;
        }
    }

    /// 34. 在排序数组中查找元素的第一个和最后一个位置
    fn binary_search(nums: &Vec<i32>, target: i32, equal: bool) -> i32 {
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

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0 as i32, nums.len() as i32 - 1);
        while l <= r {
            let p = (l + r) / 2;
            match nums[p as usize].cmp(&target) {
                std::cmp::Ordering::Equal => return p as i32,
                std::cmp::Ordering::Less => l = p + 1,
                std::cmp::Ordering::Greater => r = p - 1,
            }
        }
        l as i32
    }

    /// 36. 有效的数独
    #[allow(dead_code)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 31. 下一个排列
    #[test]
    fn next_permutation() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
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
        assert_eq!(res, true);
    }
}
