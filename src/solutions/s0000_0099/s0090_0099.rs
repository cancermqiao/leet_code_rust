pub struct Solution;

impl Solution {
    /// 97. 交错字符串
    #[allow(dead_code)]
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (n1, n2, n3) = (s1.len(), s2.len(), s3.len());
        if n1 + n2 != n3 {
            return false;
        }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = true;
        for i in 0..=n1 {
            for j in 0..=n2 {
                if i > 0 {
                    dp[i][j] |= dp[i - 1][j] && s1[i - 1] == s3[i + j - 1];
                }
                if j > 0 {
                    dp[i][j] |= dp[i][j - 1] && s2[j - 1] == s3[i + j - 1];
                }
            }
        }
        dp[n1][n2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 97. 交错字符串
    #[test]
    fn is_interleave() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        let res = Solution::is_interleave(s1, s2, s3);
        assert!(res);
    }
}
