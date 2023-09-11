pub struct Solution {}

impl Solution {
    /// 2605. 从两个数字数组里生成最小数字
    #[allow(dead_code)]
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut count = vec![0;9];
        for num in nums1 {
            count[num as usize - 1] += 1;
        }
        for num in nums2 {
            if count[num as usize - 1] == 1 {
                count[num as usize - 1] = 2;
            } else {
                count[num as usize - 1] -= 1;
            }
        }
        let (mut res, mut flag) = (0, 0);
        for (i, &n) in count.iter().enumerate() {
            if n == 2 {
                return i as i32 + 1;
            }
            if flag == 0 && n != 0 {
                res += (i as i32 + 1) * 10;
                flag = n;
            }
            if n != 0 && n == - flag {
                res += i as i32 + 1;
                flag = -2;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2605. 从两个数字数组里生成最小数字
    #[test]
    fn min_number() {
        let nums1 = vec![4,1,3];
        let nums2 = vec![5,7];
        let res = Solution::min_number(nums1, nums2);
        assert_eq!(res, 15);
    }
}