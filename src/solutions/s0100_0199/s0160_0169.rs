pub struct Solution {}

impl Solution {
    // 167. 两数之和 II - 输入有序数组
    #[allow(dead_code)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        while i < j {
            let add = numbers[i] + numbers[j];
            if add == target {
                return vec![i as i32 + 1, j as i32 + 1]
            } else if add > target {
                j -= 1;
            } else {
                i += 1;
            }
        }
        vec![i as i32 + 1, j as i32 + 1]
    }

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

    // 167. 两数之和 II - 输入有序数组
    #[test]
    fn two_sum() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let res = Solution::two_sum(nums, target);
        assert_eq!(res, vec![1, 2]);
    }

    // 169. 多数元素
    #[test]
    fn majority_element() {
        let nums = vec![2,2,1,1,1,2,2];
        let res = Solution::majority_element(nums);
        assert_eq!(res, 2);
    }
}