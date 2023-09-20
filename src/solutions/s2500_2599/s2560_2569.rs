pub struct Solution {}

impl Solution {
    /// 2560. 打家劫舍 IV
    ///
    /// 在单房间最大金额限制下，可以打劫的最多房间数
    fn rob_num(nums: &Vec<i32>, max_money: i32) -> i32 {
        let (mut count, mut visited) = (0, false);
        for &num in nums {
            if num <= max_money && !visited {
                count += 1;
                visited = true;
            } else {
                visited = false;
            }
        }
        count
    }

    #[allow(dead_code)]
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut r) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
        while l < r {
            let mid = (l + r) / 2;
            let rob_num = Self::rob_num(&nums, mid);
            if rob_num < k {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2560. 打家劫舍 IV
    #[test]
    fn min_capability() {
        let nums = vec![2, 3, 5, 9];
        let k = 2;
        let res = Solution::min_capability(nums, k);
        assert_eq!(res, 5);
    }
}
