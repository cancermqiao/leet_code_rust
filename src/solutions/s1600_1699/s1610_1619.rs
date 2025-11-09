pub struct Solution;

impl Solution {
    /// 1611. 使整数变为 0 的最少操作次数
    pub fn min_operations(n: i32) -> i32 {
        let (mut ans, mut sign) = (0, 1);
        for i in (0..30).rev() {
            if n & (1 << i) > 0 {
                ans += sign * ((1 << (i + 1)) - 1);
                sign = -sign;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1611. 使整数变为 0 的最少操作次数
    #[test]
    fn min_operations() {
        let n = 3;
        let res = Solution::min_operations(n);
        assert_eq!(res, 2);
    }
}
