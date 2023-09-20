pub struct Solution {}

impl Solution {
    /// 560. 和为k的子数组
    #[allow(dead_code)]
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum == k {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 560. 和为k的子数组
    #[test]
    fn subarray_sum() {
        let nums = vec![1,1,1];
        let k = 2;
        let res = Solution::subarray_sum(nums, k);
        assert_eq!(res, 2);
    }
}