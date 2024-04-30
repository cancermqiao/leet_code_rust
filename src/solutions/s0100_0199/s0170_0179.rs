pub struct Solution;

impl Solution {
    /// 172. 阶乘后的零
    #[allow(dead_code)]
    pub fn trailing_zeroes(n: i32) -> i32 {
        let (mut n, mut res) = (n, 0);
        while n > 0 {
            n /= 5;
            res += n;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 172. 阶乘后的零
    #[test]
    fn trailing_zeroes() {
        let n: i32 = 3;
        let res = Solution::trailing_zeroes(n);
        assert_eq!(res, 0);
    }
}