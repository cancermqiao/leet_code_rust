pub struct Solution {}

impl Solution {
    /// 198. 打家劫舍
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut pre, mut cur) = (0, 0);
        for num in nums {
            let tmp = cur;
            cur = pre + num;
            pre = pre.max(tmp);
        }
        cur.max(pre)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 198. 打家劫舍
    #[test]
    fn rob() {
        let nums = vec![1,2,3,1];
        let res = Solution::rob(nums);
        assert_eq!(res, 4);
    }
}