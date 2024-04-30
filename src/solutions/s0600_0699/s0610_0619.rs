pub struct Solution;

impl Solution {
    /// 611. 有效三角形的个数
    #[allow(dead_code)]
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut res = 0;
        for i in (2..nums.len()).rev() {
            let (mut l, mut r) = (0, i - 1);
            while l < r {
                if nums[l] + nums[r] > nums[i] {
                    res += r - l;
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 611. 有效三角形的个数
    #[test]
    fn triangle_number() {
        let nums = vec![2, 2, 3, 4];
        let res = Solution::triangle_number(nums);
        assert_eq!(res, 3);
    }
}
