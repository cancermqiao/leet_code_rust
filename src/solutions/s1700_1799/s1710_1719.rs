pub struct Solution;

impl Solution {
    /// 1716. 计算力扣银行的钱
    pub fn total_money(n: i32) -> i32 {
        let week = 1 + 2 + 3 + 4 + 5 + 6 + 7;
        let (week_num, day_num) = (n / 7, n % 7);
        week_num * week
            + week_num * (week_num - 1) / 2 * 7
            + day_num * (day_num + 1) / 2
            + week_num * day_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1716. 计算力扣银行的钱
    #[test]
    fn total_money() {
        let n = 4;
        let res = Solution::total_money(n);
        assert_eq!(res, 10);
    }
}
