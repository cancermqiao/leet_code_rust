pub struct Solution;

impl Solution {
    /// 3354. 使数组元素等于零
    #[allow(dead_code)]
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let (mut left_sum, all_sum): (i32, i32) = (0, nums.iter().sum());
        let mut res = 0;
        for num in nums {
            if num == 0 {
                let diff = (all_sum - 2 * left_sum).abs();
                if diff <= 1 {
                    res += if diff == 0 { 2 } else { 1 };
                }
            }
            left_sum += num;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 3354. 使数组元素等于零
    #[test]
    fn count_valid_selections() {
        let nums = vec![1, 0, 2, 0, 3];
        let res = Solution::count_valid_selections(nums);
        assert_eq!(res, 2);
    }
}
