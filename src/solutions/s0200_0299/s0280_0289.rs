pub struct Solution {}

impl Solution {
    // 283. 移动零
    #[allow(dead_code)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_flag = 0_usize;
        for index in 0..nums.len() {
            if nums[index] != 0 {
                nums[zero_flag] = nums[index];
                nums[index] = 0;
                zero_flag += 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // 283. 移动零
    #[test]
    fn move_zeroes() {
        let mut nums = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1,3,12,0,0]);
    }
}