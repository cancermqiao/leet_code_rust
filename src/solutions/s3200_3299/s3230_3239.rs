pub struct Solution;

impl Solution {
    /// 3234. 统计 1 显著的字符串的数量
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        for i in 0..s.len() {
            let mut cnt = [0; 2];
            for j in i..s.len() {
                cnt[(s[j] - b'0') as usize] += 1;
                if cnt[0] >= cnt[1] {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn number_of_substrings() {
        let s = "101101".to_string();
        let res = Solution::number_of_substrings(s);
        assert_eq!(res, 16);
    }
}
