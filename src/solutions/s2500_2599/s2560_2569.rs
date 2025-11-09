pub struct Solution;

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

    /// 2562. 找出数组的串联值
    pub fn find_the_array_conc_val(mut nums: Vec<i32>) -> i64 {
        let mut res = 0;
        while !nums.is_empty() {
            if nums.len() == 1 {
                res += nums.pop().unwrap() as i64;
            } else {
                let (first, last, mut bit) = (nums[0] as i64, nums.pop().unwrap() as i64, 0);
                nums.remove(0);
                let mut key = last;
                while key > 0 {
                    key /= 10;
                    bit += 1;
                }
                res += first * 10_i64.pow(bit) + last;
            }
        }
        res
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

    /// 2562. 找出数组的串联值
    #[test]
    fn find_the_array_conc_val() {
        let nums = vec![7, 52, 2, 4];
        let res = Solution::find_the_array_conc_val(nums);
        assert_eq!(res, 596_i64);
    }
}
