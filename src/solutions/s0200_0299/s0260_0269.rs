pub struct Solution;

impl Solution {
    /// 260. 只出现一次的数字 II
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xorsum = 0;
        for num in nums.iter() {
            xorsum ^= num;
        }
        let lsb = if xorsum == i32::MIN {
            xorsum
        } else {
            xorsum & -xorsum
        };
        let (mut res1, mut res2) = (0, 0);
        for num in nums.iter() {
            if num & lsb > 0 {
                res1 ^= num;
            } else {
                res2 ^= num;
            }
        }
        vec![res1, res2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 260. 只出现一次的数字 II
    #[test]
    fn single_number() {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let res = Solution::single_number(nums);
        assert_eq!(res, vec![3, 5]);
    }
}
