pub struct Solution;

impl Solution {
    /// 120. 三角形最小路径和
    #[allow(dead_code)]
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for level in (0..triangle.len() - 1).rev() {
            for i in 0..triangle[level].len() {
                triangle[level][i] += triangle[level + 1][i].min(triangle[level + 1][i + 1]);
            }
        }
        triangle[0][0]
    }

    /// 121. 买卖股票的最佳时机
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

    /// 122. 买卖股票的最佳时机 II
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

    /// 123. 买卖股票的最佳时机 III
    #[allow(dead_code)]
    pub fn max_profit_v3(prices: Vec<i32>) -> i32 {
        let (mut buy1, mut buy2) = (-prices[0], -prices[0]);
        let (mut sell1, mut sell2) = (0, 0);
        for price in &prices[1..] {
            buy1 = buy1.max(-price);
            sell1 = sell1.max(buy1 + price);
            buy2 = buy2.max(sell1 - price);
            sell2 = sell2.max(buy2 + price);
        }
        sell2
    }

    /// 125. 验证回文串
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

    /// 128. 最长连续序列
    #[allow(dead_code)]
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: std::collections::HashSet<i32> = nums.into_iter().collect();
        let mut longest_length = 0;
        for num in nums.iter() {
            if !nums.contains(&(num - 1)) {
                let mut cur_num = *num;
                let mut length = 1;
                while nums.contains(&(cur_num + 1)) {
                    length += 1;
                    cur_num += 1;
                }
                longest_length = longest_length.max(length);
            }
        }
        longest_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 120. 三角形最小路径和
    #[test]
    fn minimum_total() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let res = Solution::minimum_total(triangle);
        assert_eq!(res, 11);
    }

    /// 121. 买卖股票的最佳时机
    #[test]
    fn max_profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let profit = Solution::max_profit(prices);
        assert_eq!(profit, 5);
    }

    /// 122. 买卖股票的最佳时机 II
    #[test]
    fn max_profit_v2() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let profit = Solution::max_profit_v2(prices);
        assert_eq!(profit, 7);
    }

    /// 123. 买卖股票的最佳时机 III
    #[test]
    fn max_profit_v3() {
        let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
        let res = Solution::max_profit_v3(prices);
        assert_eq!(res, 6);
    }

    /// 125. 验证回文串
    #[test]
    fn is_palindrome() {
        let s = "A man, a plan, a canal: Panama".to_string();
        let res = Solution::is_palindrome(s);
        assert!(res);
    }

    /// 128. 最长连续序列
    #[test]
    fn longest_consecutive() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let res = Solution::longest_consecutive(nums);
        assert_eq!(res, 4);
    }
}
