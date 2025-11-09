pub struct Solution;


impl Solution {
    /// 2169. 得到 0 的操作数
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut ans = 0;
        while num1 != 0 && num2 != 0 {
            if num1 >= num2 {
                ans += num1 / num2;
                num1 %= num2;
            } else {
                ans += num2 / num1;
                num2 %= num1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2125. 银行中的激光束数量
    #[test]
    pub fn count_operations() {
        let (num1, num2) = (2, 3);
        let res = Solution::count_operations(num1, num2);
        assert_eq!(res, 3);
    }
}

