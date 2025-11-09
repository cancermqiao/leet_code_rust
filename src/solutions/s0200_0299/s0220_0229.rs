pub struct Solution;

impl Solution {
    /// 228. 汇总区间
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() && nums[j - 1] + 1 == nums[j] {
                j += 1;
            }
            if j - 1 == i {
                res.push(nums[i].to_string());
            } else {
                res.push(format!("{}->{}", nums[i], nums[j - 1]));
            }
            i = j;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 228. 汇总区间
    #[test]
    fn summary_ranges() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let res = Solution::summary_ranges(nums);
        assert_eq!(res, vec!["0->2", "4->5", "7"]);
    }
}
