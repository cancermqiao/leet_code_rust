pub struct Solution {}

impl Solution {
    /// 2235. 两整数相加
    #[allow(dead_code)]
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// 2235. 两整数相加
    #[test]
    fn sum() {
        let num1 = 12;
        let num2 = 5;
        let res = Solution::sum(num1, num2);
        assert_eq!(res, 17);
    }
}