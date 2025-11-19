pub struct Solution;

impl Solution {
    /// 1437. 是否所有 1 都至少相隔 k 个元素
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut zero_cnt = -1;
        for &num in nums.iter() {
            if num == 1 {
                if zero_cnt >= 0 && zero_cnt < k {
                    return false;
                }
                zero_cnt = 0;
            } else if zero_cnt >= 0{
                zero_cnt += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k_length_apart() {
        let nums = vec![1, 0, 0, 0, 1, 0, 0, 1];
        let k = 2;
        let res = Solution::k_length_apart(nums, k);
        assert!(res);
    }
}
