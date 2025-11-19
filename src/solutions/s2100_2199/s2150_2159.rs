pub struct Solution;

impl Solution {
    /// 2154. 将找到的值乘以 2
    pub fn find_final_value(nums: Vec<i32>, mut original: i32) -> i32 {
        while nums.contains(&original) {
            original *= 2;
        }
        original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_final_value() {
        let nums = vec![5, 3, 6, 1, 12];
        let original = 3;
        let res = Solution::find_final_value(nums, original);
        assert_eq!(res, 24);
    }
}
