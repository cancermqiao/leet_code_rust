pub struct Solution;

impl Solution {
    /// 3370. 仅含置位位的最小整数
    pub fn smallest_number(n: i32) -> i32 {
        let bit_cnt = 32 - n.leading_zeros();
        ((1 << bit_cnt) - 1) | n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 3370. 仅含置位位的最小整数
    #[test]
    fn smallest_number() {
        let n: i32 = 5;
        let res = Solution::smallest_number(n);
        assert_eq!(res, 7);
    }
}
