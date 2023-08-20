pub struct Solution {}

impl Solution {
    // 121. 买卖股票的最佳时机
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut buy_price = prices[0];
        for price in prices {
            profit = profit.max(price - buy_price);
            buy_price = buy_price.min(price);
        }
        profit
    }

    // 122. 买卖股票的最佳时机 II
    #[allow(dead_code)]
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut pre = prices[0];
        for price in prices {
            if price > pre {
                profit += price - pre;
            }
            pre = price;
        }
        profit
    }

    // 125. 验证回文串
    #[allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s.chars().filter(|x| x.is_alphanumeric());
        while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
            if a.to_ascii_lowercase() != b.to_ascii_lowercase() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 121. 买卖股票的最佳时机
    #[test]
    fn max_profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let profit = Solution::max_profit(prices);
        assert_eq!(profit, 5);
    }

    // 122. 买卖股票的最佳时机 II
    #[test]
    fn max_profit_v2() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let profit = Solution::max_profit_v2(prices);
        assert_eq!(profit, 7);
    }

    // 125. 验证回文串
    #[test]
    fn is_palindrome() {
        let s = "A man, a plan, a canal: Panama".to_string();
        let res = Solution::is_palindrome(s);
        assert_eq!(res, true);
    }
}