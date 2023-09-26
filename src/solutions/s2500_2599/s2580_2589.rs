pub struct Solution {}

impl Solution {
    /// 2582. 递枕头
    #[allow(dead_code)]
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        if time / (n - 1) % 2 == 0 {
            1 + time % (n - 1)
        } else {
            n - time % (n - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2582. 递枕头
    #[test]
    fn pass_the_pillow() {
        let n = 4;
        let time = 5;
        let res = Solution::pass_the_pillow(n, time);
        assert_eq!(res, 2);
    }
}
