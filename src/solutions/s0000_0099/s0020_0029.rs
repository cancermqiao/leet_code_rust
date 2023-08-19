pub struct Solution {}

impl Solution {
    // 26. 删除有序数组中的重复项
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut p = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[p] {
                nums[p + 1] = nums[i];
                p += 1;
            }
        }
        (p + 1) as i32
    }

    // 27. 移除元素
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut p = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[p] = nums[i];
                p += 1;
            }
        }
        p as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 26. 移除元素
    #[test]
    fn remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        let res = Solution::remove_duplicates(&mut nums);
        assert_eq!(&nums[0..res as usize], [1, 2])
    }

    // 27. 移除元素
    #[test]
    fn remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let res = Solution::remove_element(&mut nums, val);
        assert_eq!(&nums[0..res as usize], [2, 2])
    }
}
