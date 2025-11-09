pub struct Solution;

impl Solution {
    /// LCP 06. 拿硬币
    pub fn min_count(coins: Vec<i32>) -> i32 {
        let mut res = 0;
        for coin in coins {
            res += coin / 2;
            if coin % 2 == 1 {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// LCP 06. 拿硬币
    #[test]
    fn min_count() {
        let coins = vec![4, 2, 1];
        let res = Solution::min_count(coins);
        assert_eq!(res, 4);
    }
}
