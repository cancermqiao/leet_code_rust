pub struct Solution;

impl Solution {
    /// 1513. 仅含 1 的子串数
    pub fn num_sub(s: String) -> i32 {
        let (mut ans, mut cnt) = (0i64, 0i64);
        s.as_bytes().iter().for_each(|&c| {
            if c == b'1' {
                cnt += 1;
            } else {
                ans = (ans + (cnt * (cnt + 1)) / 2) % 1_000_000_007;
                cnt = 0;
            }
        });
        ((ans + (cnt * (cnt + 1)) / 2) % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn num_sub() {
        let s = "0110111".to_string();
        let res = Solution::num_sub(s);
        assert_eq!(res, 9);
    }
}
