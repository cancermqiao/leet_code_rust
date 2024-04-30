pub struct Solution;

impl Solution {
    /// 238. 除自身以外数组的乘积
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        nums[0..nums.len()-1].iter().enumerate().for_each(|(index, &num)| {
            res[index+1] = res[index] * num;
        });
        let mut cum = 1;
        nums.iter().enumerate().rev().for_each(|(index, &num)| {
            res[index] *= cum;
            cum *= num;
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 238. 除自身以外数组的乘积
    #[test]
    fn product_except_self() {
        let nums = vec![1,2,3,4];
        let res = Solution::product_except_self(nums);
        assert_eq!(res, vec![24,12,8,6]);
    }
}