pub struct Solution;

impl Solution {
    /// 3289. 数字小镇中的捣蛋鬼
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 - 3;
        let sum: f64 = nums.iter().map(|&x| x as f64).sum();
        let square_sum: f64 = nums.iter().map(|&x| (x * x) as f64).sum();
        let sneaky_sum = sum - (n * (n + 1) / 2) as f64;
        let sneaky_square_sum = square_sum - (n * (n + 1) * (2 * n + 1) / 6) as f64;
        let delta = (2.0 * sneaky_square_sum - sneaky_sum * sneaky_sum).sqrt();
        vec![
            ((sneaky_sum - delta) / 2.0) as i32,
            ((sneaky_sum + delta) / 2.0) as i32,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 3289. 数字小镇中的捣蛋鬼
    #[test]
    fn get_sneaky_numbers() {
        let nums = vec![0, 1, 1, 0];
        let res = Solution::get_sneaky_numbers(nums);
        assert_eq!(res, vec![0, 1]);
    }
}
