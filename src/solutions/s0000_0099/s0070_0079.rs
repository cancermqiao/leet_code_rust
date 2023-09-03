pub struct Solution {}

impl Solution {
    /// 70. 爬楼梯
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut pre, mut prepre) = (1, 0);
        for _ in 0..n {
            let tmp = pre;
            pre = pre + prepre;
            prepre = tmp;
        }
        pre
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 70. 爬楼梯
    #[test]
    fn climb_stairs() {
        let n = 2;
        let res = Solution::climb_stairs(n);
        assert_eq!(res, 2);
    }
}