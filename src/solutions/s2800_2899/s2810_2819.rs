pub struct Solution;

impl Solution {
    /// 2810. 故障键盘
    #[allow(dead_code)]
    pub fn final_string(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            if c == 'i' {
                res = res.chars().rev().collect()
            } else {
                res.push(c);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_string() {
        let s = "string".to_string();
        let res = Solution::final_string(s);
        assert_eq!(res, "rtsng".to_string());
    }
}