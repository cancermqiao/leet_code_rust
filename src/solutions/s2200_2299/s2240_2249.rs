pub struct Solution;

impl Solution {
    /// 2240. 买钢笔和铅笔的方案数
    pub fn ways_to_buy_pens_pencils(mut total: i32, cost1: i32, cost2: i32) -> i64 {
        if cost1 < cost2 {
            return Self::ways_to_buy_pens_pencils(total, cost2, cost1);
        }
        let mut res = 0_i64;
        while total >= 0 {
            res += (total / cost2 + 1) as i64;
            total -= cost1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2240. 买钢笔和铅笔的方案数
    #[test]
    pub fn ways_to_buy_pens_pencils() {
        let total = 20;
        let cost1 = 10;
        let cost2 = 5;
        let res = Solution::ways_to_buy_pens_pencils(total, cost1, cost2);
        assert_eq!(res, 9);
    }
}
