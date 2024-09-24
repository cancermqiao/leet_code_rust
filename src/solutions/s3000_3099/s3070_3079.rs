pub struct Solution;

impl Solution {
    /// 3075. 幸福值最大化的选择方案
    #[allow(dead_code)]
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable();
        let mut max_happiness = 0;
        let n = happiness.len();
        for i in 0..k {
            max_happiness += (happiness[n-i as usize-1] - i).max(0) as i64;
        }
        max_happiness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_happiness_sum() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        let res = Solution::maximum_happiness_sum(happiness, k);
        assert_eq!(res, 4);
    }
}