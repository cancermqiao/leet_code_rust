pub struct Solution;

impl Solution {
    /// 322. 零钱兑换
    #[allow(dead_code)]
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount+1; amount as usize + 1];
        dp[0] = 0;
        for coin in coins {
            for x in coin..amount+1 {
                dp[x as usize] = dp[x as usize].min(dp[(x - coin) as usize] + 1);
            }
        }
        if dp[amount as usize] == amount + 1{
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 322. 零钱兑换
    #[test]
    fn coin_change() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        let res = Solution::coin_change(coins, amount);
        assert_eq!(res, 3);
    }
}