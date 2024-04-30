pub struct Solution;

impl Solution {
    /// 1402. 做菜顺序
    #[allow(dead_code)]
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_by(|a, b| b.cmp(a));
        let (mut sum, mut res) = (0, 0);
        for s in &satisfaction {
            sum += s;
            if sum <= 0 {
                return res;
            }
            res += sum;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1402. 做菜顺序
    #[test]
    fn max_satisfaction() {
        let satisfaction = vec![-1, -8, 0, 5, -9];
        let res = Solution::max_satisfaction(satisfaction);
        assert_eq!(res, 14);
    }
}
