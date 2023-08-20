pub struct Solution {}

impl Solution {
    // 45. 跳跃游戏 II
    #[allow(dead_code)]
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut max_pos, mut step, mut end) = (0, 0, 0);
        for i in 0..nums.len() - 1 {
            max_pos = max_pos.max(i as i32 + nums[i]);
            if max_pos >= nums.len() as i32 - 1 {
                return step + 1;
            }
            if i as i32 == end {
                step += 1;
                end = max_pos;
            }
            
        }
        return step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 45. 跳跃游戏 II
    #[test]
    fn jump() {
        let nums = vec![2,3,1,1,4];
        let step = Solution::jump(nums);
        assert_eq!(step, 2);
    }
}
