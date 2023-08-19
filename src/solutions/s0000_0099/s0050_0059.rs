pub struct Solution {}

impl Solution {
    // 55. 跳跃游戏
    #[allow(dead_code)]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut step = 1;
        for i in (0..(nums.len() - 1)).rev() {
            if nums[i] < step {
                step += 1;
            } else {
                step = 1;
            }
        }
        step == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 55. 跳跃游戏
    #[test]
    fn can_jump() {
        let nums = vec![2, 3, 1, 1, 4];
        let res = Solution::can_jump(nums);
        assert_eq!(res, true);

        let nums = vec![3, 2, 1, 0, 4];
        let res = Solution::can_jump(nums);
        assert_eq!(res, false);
    }
}
