pub struct Solution {}

impl Solution {
    /// 215. 数组中的第k个最大元素
    #[allow(dead_code)]
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        fn helper(nums: &mut Vec<i32>, l: usize, r: usize, k: i32) -> i32 {
            let pivot = l;
            let (mut i, mut j) = (l, r);
            while i < j {
                while i < j && nums[pivot] >= nums[j] {
                    j -= 1
                }
                while i < j && nums[pivot] <= nums[i] {
                    i += 1
                }
                nums.swap(i, j);
            }
            nums.swap(i, pivot);

            if i as i32 + 1 == k {
                nums[i]
            } else if i as i32 + 1 > k {
                helper(nums, l, i - 1, k)
            } else {
                helper(nums, i + 1, r, k)
            }
        }
        let n = nums.len();
        helper(&mut nums, 0, n - 1, k)
    }

    /// 219. 存在重复元素 II
    #[allow(dead_code)]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut last_index = std::collections::HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(v) = last_index.get(num) {
                if v + k as usize >= i {
                    return true;
                }
            }
            last_index.insert(num, i);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 215. 数组中的第k个最大元素
    #[test]
    fn find_kth_largest() {
        let nums = vec![3,2,3,1,2,4,5,5,6];
        let k = 4;
        let res = Solution::find_kth_largest(nums, k);
        assert_eq!(res, 4);
    }

    /// 219. 存在重复元素 II
    #[test]
    fn contains_nearby_duplicate() {
        let nums = vec![1,2,3,1];
        let k = 3;
        let res = Solution::contains_nearby_duplicate(nums, k);
        assert_eq!(res, true);
    }
}
