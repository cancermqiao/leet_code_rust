pub struct Solution;

impl Solution {
    /// 80. 删除有序数组中的重复项 II
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let k = 2;
        if nums.len() <= k {
            return nums.len() as i32;
        }

        let mut p = k;
        for i in k..nums.len() {
            if nums[i] != nums[p-k] {
                nums[p] = nums[i];
                p += 1;
            }
        }
        p as i32
    }

    /// 88. 合并两个有序数组
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        while i >= 0 || j >= 0 {
            if i < 0 {
                nums1[(i+j+1) as usize] = nums2[j as usize];
                j -= 1;
            } else if j < 0 {
                nums1[(i+j+1) as usize] = nums1[i as usize];
                i -= 1;
            } else if nums1[i as usize] < nums2[j as usize] {
                nums1[(i+j+1) as usize] = nums2[j as usize];
                j -= 1;
            } else {
                nums1[(i+j+1) as usize] = nums1[i as usize];
                i -= 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 80. 删除有序数组中的重复项 II
    #[test]
    fn remove_duplicates() {
        let mut nums = vec![1,1,1,2,2,3];
        let res = Solution::remove_duplicates(&mut nums);
        assert_eq!(&nums[0..res as usize], vec![1,1,2,2,3]);
    }

    /// 88. 合并两个有序数组
    #[test]
    fn merge() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1,2,2,3,5,6]);
    }
}