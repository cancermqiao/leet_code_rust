pub struct Solution {}

impl Solution {
    /// 66. 加一
    #[allow(dead_code)]
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut remaind = 1;
        for i in (0..digits.len()).rev() {
            if remaind == 0 {
                return digits
            }
            let sum = digits[i] + remaind;
            if sum == 10 {
                remaind = 1;
                digits[i] = 0;
            } else {
                remaind = 0;
                digits[i] = sum;
            }
        }
        if remaind == 1 {
            digits.insert(0, remaind)
        }
        digits
    }

    /// 69. x的平方根
    ///
    /// # 二分查找
    #[allow(dead_code)]
    fn binary_search(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let (mut l, mut r, mut res) = (0, x, -1);
        while l <= r {
            let p = (l + r) / 2;
            if x / p >= p {
                res = p;
                l = p + 1;
            } else {
                r = p - 1;
            }
        }
        res
    }

    /// # 牛顿法
    #[allow(dead_code)]
    pub fn newton_method(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut x0 = x as f64 / 2.;
        loop {
            let x1 = 0.5 * (x as f64 / x0 + x0);
            if (x1 - x0).abs() < 1e-6 {
                break;
            }
            x0 = x1;
        }
        x0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 66. 加一
    #[test]
    fn plus_one() {
        let digits = vec![1, 2, 3];
        let res = Solution::plus_one(digits);
        assert_eq!(res, vec![1, 2, 4]);
    }

    /// 69. x的平方根
    #[test]
    pub fn my_sqrt() {
        let x = 4;
        let res = Solution::binary_search(x);
        assert_eq!(res, 2);

        let x = 8;
        let res = Solution::newton_method(x);
        assert_eq!(res, 2);
    }
}
