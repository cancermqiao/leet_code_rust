pub struct Solution {}

impl Solution {
    /// 392. 判断子序列
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut s_c = s_chars.next();
        for t in t.chars() {
            match s_c {
                Some(s) => {
                    if s == t {
                        s_c = s_chars.next();
                    }
                }
                None => return true,
            }
        }
        s_c.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 392. 判断子序列
    #[test]
    fn is_subsequence() {
        let (s, t) = ("abc".to_string(), "ahbgdc".to_string());
        let res = Solution::is_subsequence(s, t);
        assert!(res);
    }
}
