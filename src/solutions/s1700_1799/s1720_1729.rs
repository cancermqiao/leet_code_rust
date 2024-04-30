pub struct Solution;

impl Solution {
    /// 1726. 同积元祖
    #[allow(dead_code)]
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut cnt_map = std::collections::HashMap::new();
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                cnt_map
                    .entry(nums[i] * nums[j])
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
        cnt_map.values().map(|c| c * (c - 1) * 4).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1726. 同积元祖
    #[test]
    fn tuple_same_product() {
        let nums = vec![2, 3, 4, 6];
        let res = Solution::tuple_same_product(nums);
        assert_eq!(res, 8);
    }
}
