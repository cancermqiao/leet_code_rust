pub struct Solution {}

impl Solution {
    // 169. 多数元素
    #[allow(dead_code)]
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut count = 1;
        for num in &nums[1..nums.len()] {
            if res != *num {
                if count > 0{
                    count -= 1;
                } else {
                    res = *num;
                }
            } else {
                count += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 169. 多数元素
    #[test]
    fn majority_element() {
        let nums = vec![2,2,1,1,1,2,2];
        let res = Solution::majority_element(nums);
        assert_eq!(res, 2)
    }
}