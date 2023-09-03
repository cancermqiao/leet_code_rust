pub struct Solution {}

impl Solution {
    /// 209. 长度最小的子数组
    #[allow(dead_code)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut i, mut sum, mut res) = (0, 0, target + 1);
        for j in 0..nums.len() {
            sum += nums[j];
            while sum >= target {
                res = res.min((j - i + 1) as i32);
                sum -= nums[i];
                i += 1;
            }
        }
        if res == target + 1 {
            0
        } else {
            res as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 209. 长度最小的子数组
    #[test]
    fn min_sub_array_len() {
        let target = 7;
        let nums = vec![2,3,1,2,4,3];
        let res = Solution::min_sub_array_len(target, nums);
        assert_eq!(res, 2);
    }
}