use crate::data::list::ListNode;

pub struct Solution {}

impl Solution {
    /// 60. 排列序列
    #[allow(dead_code)]
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut nums: Vec<String> = (1..=n as i32).map(|x| x.to_string()).collect();
        let (mut cnts, mut multi) = (Vec::new(), 1);
        for i in 1..n as i32 {
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
    use crate::data::list::List;

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
        let vectors = List { head: res }.to_vec();
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
